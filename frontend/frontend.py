"""Frontend application"""
import asyncio
import logging
import os
import sys
from logging.handlers import TimedRotatingFileHandler
from time import monotonic
from typing import Any, Dict, List, Optional, Set

from colorlog import ColoredFormatter

from frontend.base.core_client import CoreClient
from frontend.base.oauth import CoreOauth2Client
from frontend.base.state import FrontendState
from frontend.config import Config
from frontend.webserver import Webserver

ERROR_LOG_FILENAME = "frontend.log"

# How long to wait to log tasks that are blocking
BLOCK_LOG_TIMEOUT = 60  # seconds

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Frontend:
    """Frontend application class."""

    http: Webserver = None
    oauth_client: CoreOauth2Client = None
    core_client: CoreClient = None

    def __init__(self, config: Config) -> None:
        self.loop = asyncio.new_event_loop()
        self.config: Config = config
        self.state: FrontendState = FrontendState.not_running
        self.exit_code: int = 0

        # If not None, use to signal end-of-loop
        self._stopped: Optional[asyncio.Event] = None

        self._pending_tasks: List = []

    @property
    def is_running(self) -> bool:
        """Return if frontend is running."""
        return self.state in (FrontendState.starting, FrontendState.running)

    @property
    def is_stopping(self) -> bool:
        """Return if frontend is stopping."""
        return self.state in (FrontendState.stopping, FrontendState.final_write)

    def async_enable_logging(self, verbose: bool = False) -> None:
        """Set up logging for frontend."""
        fmt = "%(asctime)s %(levelname)s (%(threadName)s) [%(name)s] %(message)s"
        datefmt = "%Y-%m-%d %H:%M:%S"
        logging.basicConfig(level=logging.INFO)

        colorfmt = f"%(log_color)s{fmt}%(reset)s"
        logging.getLogger().handlers[0].setFormatter(
            ColoredFormatter(
                colorfmt,
                datefmt=datefmt,
                reset=True,
                log_colors={
                    "DEBUG": "cyan",
                    "INFO": "green",
                    "WARNING": "yellow",
                    "ERROR": "red",
                    "CRITICAL": "red",
                },
            )
        )

        logging.basicConfig(format=fmt, datefmt=datefmt, level=logging.INFO)

        logging.getLogger("requests").setLevel(logging.WARNING)
        logging.getLogger("urllib3").setLevel(logging.WARNING)
        logging.getLogger("aiohttp.access").setLevel(logging.WARNING)

        sys.excepthook = lambda *args: logging.getLogger("").exception(
            "Uncaught exception", exc_info=args  # type: ignore
        )
        log_rotate_days = 14

        err_log_path = os.path.join(os.getcwd(), ERROR_LOG_FILENAME)
        err_dir = os.path.dirname(err_log_path)
        if not err_dir:
            os.mkdir(err_dir)
        err_handler: logging.FileHandler = TimedRotatingFileHandler(
            err_log_path, when="midnight", backupCount=log_rotate_days
        )
        err_handler.setLevel(logging.DEBUG if verbose else logging.WARNING)
        err_handler.setFormatter(logging.Formatter(fmt, datefmt=datefmt))

        logger = logging.getLogger("")
        logger.addHandler(err_handler)
        logger.setLevel(logging.DEBUG if verbose else logging.WARNING)

    async def start(self) -> int:
        """Start frontend application.
        Note: This function is only used for testing.
        For regular use, use "await photos.run()".
        """
        _LOGGER.debug("start core loop")
        await self.async_block_till_done()

        return self.exit_code

    async def async_run(self, *, attach_signals: bool = True) -> int:
        """Run the application.
        Main entry point of the frontend application.
        """
        if self.state != FrontendState.not_running:
            raise RuntimeError("Frontend is already running")

        # _async_stop will set this instead of stopping the loop
        self._stopped = asyncio.Event()

        await self.async_start()

        await self._stopped.wait()
        return self.exit_code

    async def async_start(self) -> None:
        """Finalize startup from inside the event loop."""
        self.state = FrontendState.starting

        try:
            await self.async_block_till_done()
        except asyncio.TimeoutError:
            _LOGGER.warning(
                "Something is blocking frontend from wrapping up the start up phase. We're going to continue anyway."
            )

        # Wait for all startup triggers before changing state
        await asyncio.sleep(0)

        if self.state != FrontendState.starting:
            _LOGGER.warning("Frontend startup has been interrupted. Its state may be inconsistent")
            return

        self.state = FrontendState.running

    async def async_block_till_done(self) -> None:
        """Block until all pending work is done."""
        # To flush out any call_soon_threadsafe
        await asyncio.sleep(0)

        # setup oauth client
        self.oauth_client = CoreOauth2Client(self.config.client_id, self.config.client_secret)
        self.oauth_client.set_from_config(
            config=self.config,
            logger=_LOGGER,
        )
        _LOGGER.info("Oauth client setup done.")

        self.core_client = CoreClient(self.config)
        _LOGGER.info("Core client setup done.")

        # setup webserver
        self.http = Webserver(self)
        await self.http.start()
        _LOGGER.info("Webserver should be up and running...")

        # iterate through pending tasks
        while self._pending_tasks:
            pending = [task for task in self._pending_tasks if not task.done()]
            self._pending_tasks.clear()
            if pending:
                await self._await_and_log_pending(pending)

                if start_time is None:
                    # Avoid calling monotonic() until we know
                    # we may need to start logging blocked tasks.
                    start_time = 0
                elif start_time == 0:
                    # If we have waited twice then we set the start time
                    start_time = monotonic()
                elif monotonic() - start_time > BLOCK_LOG_TIMEOUT:
                    # We have waited at least three loops and new tasks
                    # continue to block. At this point we start
                    # logging all waiting tasks.
                    for task in pending:
                        _LOGGER.debug("Waiting for task: %s", task)
            else:
                await asyncio.sleep(0)

    async def async_stop(self) -> None:
        """Stop Photos.network core application."""
        self.state = FrontendState.stopping
        self.state = FrontendState.final_write
        self.state = FrontendState.not_running
        self.state = FrontendState.stopped

        if self._stopped is not None:
            self._stopped.set()
