import importlib.resources as importlib_resources
import logging
import os
import time
from ipaddress import ip_address
from typing import TYPE_CHECKING, Any, Awaitable, Callable, Dict

import aiohttp_jinja2
import aiohttp_session
import jinja2
from aiohttp import web
from aiohttp.web_middlewares import middleware
from aiohttp_session import session_middleware
from aiohttp_session.cookie_storage import EncryptedCookieStorage

from frontend.const import KEY_AUTHENTICATED
from frontend.i18n import i18n
from frontend.views.callback import OauthCallbackView
from frontend.views.index import IndexView
from frontend.views.login import LoginView
from frontend.views.logout import LogoutView
from frontend.views.photo import PhotoView
from frontend.views.settings import SettingsView
from frontend.views.user import UserView

if TYPE_CHECKING:
    from frontend.frontend import Frontend

MAX_CLIENT_SIZE: int = 1024**2 * 16

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Webserver:
    def __init__(self, frontend: "Frontend"):
        self.frontend = frontend
        self.app = web.Application(middlewares=[], client_max_size=MAX_CLIENT_SIZE)
        self.runner = web.AppRunner(self.app)

        # check if secret length is 32 bytes
        if len(self.frontend.config.cookie_secret) == 32:
            _LOGGER.debug("use cookie_secret from file")
            secret_key = bytes(self.frontend.config.cookie_secret, "utf-8")
        else:
            _LOGGER.debug("use fallback cookie_secret")
            secret_key = bytes("Thirty  two  length  bytes  key!", "utf-8")

        # TODO: switch to Redis storage?
        aiohttp_session.setup(
            self.app,
            EncryptedCookieStorage(
                secret_key=secret_key,
                cookie_name="Photos.network",
            ),
        )

        # init jinja2 template engine
        pkg = importlib_resources.files("frontend")
        env = aiohttp_jinja2.setup(
            self.app,
            loader=jinja2.FileSystemLoader(pkg / "templates"),
            extensions=["jinja2.ext.i18n", "jinja2.ext.debug"],
            context_processors=[self.username_ctx_processor],
        )

        # setup i18n
        env.install_gettext_translations(i18n, newstyle=True)

        self.register_request(LoginView())
        self.register_request(LogoutView())
        self.register_request(OauthCallbackView())
        self.register_request(IndexView())
        self.register_request(PhotoView())
        self.register_request(UserView())
        self.register_request(SettingsView())

        # self.app.router.add_static("/static", os.path.join("frontend/static"))

        self.app.middlewares.append(self.auth_middleware)

    async def start(self):
        """Start webserver."""
        await self.runner.setup()

        frontend_port = self.frontend.config.frontend_port

        # use host=None to listen on all interfaces.
        site = web.TCPSite(runner=self.runner, host=None, port=frontend_port)
        await site.start()
        _LOGGER.info(f"Webserver is listening on {site._host}:{site._port}")

    async def stop(self):
        """Stop webserver."""
        await self.runner.cleanup()

    def register_request(self, view):
        """
        Register a request.
        The view argument must be a class that inherits from Request.
        It is optional to instantiate it before registering; this method will
        handle it either way.
        """
        # if isinstance(view, type):
        #     # Instantiate the view, if needed
        #     view = view()

        if not hasattr(view, "url"):
            class_name = view.__class__.__name__
            raise AttributeError(f'{class_name} missing required attribute "url"')

        view.register(self.frontend, self.app.router)

    async def username_ctx_processor(self, request: web.Request) -> Dict[str, Any]:
        """Jinja2 context processor to extract the username from an active session."""
        session = await aiohttp_session.get_session(request)
        username = session.get("username")

        return {"username": username}

    @middleware
    async def auth_middleware(
        self, request: web.Request, handler: Callable[[web.Request], Awaitable[web.StreamResponse]]
    ) -> web.StreamResponse:
        """Check if user is authenticated and authentication is still valid."""

        authenticated = False
        session = await aiohttp_session.get_session(request)
        username = session.get("username")

        if not username:
            return await handler(request)

        refresh_token = session.get("refresh_token")
        expires_in = session.get("expires_in")

        if expires_in is not None and refresh_token is not None:
            now = int(time.time())

            # refresh token if access_token expires in next 3 minutes or has been expired already
            if (expires_in - now) <= 180:
                _LOGGER.info("access token is expiring in " + str((expires_in - now)) + " seconds. Try to refresh...")

                (
                    statusCode,
                    accessToken,
                    refreshToken,
                    expiresIn,
                ) = await self.frontend.core_client.refresh_access_token_call()
                if statusCode is not None and statusCode == 200:
                    session["expires_in"] = expiresIn
                    session["access_token"] = str(accessToken)
                    session["refresh_token"] = str(refreshToken)

                    authenticated = True
                else:
                    authenticated = False

            else:
                # still authenticated
                authenticated = True

        validate_access_token_status = await self.frontend.core_client.check_access_token()
        if validate_access_token_status != 200:
            authenticated = False

            if self.frontend.core_client.accessToken == None:
                session.clear()
                raise web.HTTPFound(location="/")

        request[KEY_AUTHENTICATED] = authenticated

        return await handler(request)
