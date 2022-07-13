import asyncio
import os
from typing import Any, Awaitable, Callable, Dict

import aiohttp_jinja2
import aiohttp_session
import jinja2
from aiohttp import web
from aiohttp_session import session_middleware
from aiohttp_session.cookie_storage import EncryptedCookieStorage

from frontend.config import Config
from frontend.i18n import i18n


async def username_ctx_processor(request: web.Request) -> Dict[str, Any]:
    """Jinja2 context processor to extract the username from an active session."""
    session = await aiohttp_session.get_session(request)
    if "first_name" in session:
        username = session.get("first_name")
    elif "last_name" in session:
        username = session.get("last_name")
    else:
        username = session.get("username")

    return {"username": username}


def create_application(
    loop=asyncio.new_event_loop(),
    config: Config = None,
) -> web.Application:
    app = web.Application(client_max_size=64 * 1024**2)

    # check if secret length is 32 bytes
    if len(config.cookie_secret) == 32:
        secret_key = bytes(config.cookie_secret, "utf-8")
    else:
        secret_key = bytes("Thirty  two  length  bytes  key!", "utf-8")

    aiohttp_session.setup(app, EncryptedCookieStorage(secret_key))

    for route in app.routes.routes:
        app.router.add_route(*route[0], **route[1])

    for middleware in app.middlewares.middlewares:
        app.middlewares.append(middleware)

    return app
