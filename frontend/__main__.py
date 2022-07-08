"""Entry point of Photos.network frontend."""
import asyncio
import json
import logging
import os
import socket
import sys
from typing import Any, Dict
from xml.dom.expatbuilder import parseString

from aiohttp import web

from frontend.application import create_application
from frontend.config import Config
from frontend.const import REQUIRED_PYTHON_VER
from frontend.frontend import Frontend

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


def validate_python() -> None:
    """Validate that the right Python version is running."""
    if sys.version_info[:3] < REQUIRED_PYTHON_VER:
        print(
            "Photos.network requires at least Python "
            f"{REQUIRED_PYTHON_VER[0]}.{REQUIRED_PYTHON_VER[1]}.{REQUIRED_PYTHON_VER[2]}"
        )
        sys.exit(1)


def main() -> int:
    """Start Photos.network application."""
    validate_python()

    config_dir = os.path.abspath(os.path.join(os.getcwd(), "config"))
    config_file = os.path.join(config_dir, "frontend_configuration.json")

    if not os.path.exists(config_dir):
        _LOGGER.info(f"create directory {config_dir}")
        os.mkdir(config_dir)

    if not os.path.exists(config_file):
        # create default config file
        with open(file=config_file, mode="w+", encoding="utf-8") as file:
            hostname = socket.gethostname()

            output = {
                "frontend_url": "http://" + str(socket.gethostbyname(hostname)),
                "frontend_port": 7778,
                "core_url": "http://127.0.0.1",
                "core_port": 7777,
                "client_id": "",
                "client_secret": "",
                "redirect_uri": "http://127.0.0.1:7778/callback",
            }
            json.dump(output, file, indent=2)
            file.close()
            _LOGGER.info(f"default config_file {config_file} created.")
    else:
        _LOGGER.info(f"config_file found at {config_file}")

    with open(file=config_file, mode="r", encoding="utf-8") as file:
        conf_dict = json.load(file)
        file.close()

    if not isinstance(conf_dict, dict):
        msg = f"The configuration file {os.path.basename(config_dir)} does not contain a dictionary"
        _LOGGER.error(msg)
        raise RuntimeError(msg)

    config = Config.fromConfigFile(conf_dict)

    frontend = Frontend(config)
    frontend.async_enable_logging(verbose=True)

    try:
        exit_code = asyncio.run(frontend.async_run())
    except KeyboardInterrupt:
        # TODO: handle running threads
        print("### Interrupt application without taking care of running threads ###")
        exit_code = 0

    return exit_code

if __name__ == "__main__":
    sys.exit(main())
