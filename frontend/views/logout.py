import logging
from hashlib import sha1
from random import SystemRandom
from typing import TYPE_CHECKING

import aiohttp_session
from aiohttp import web
from frontend.base.request import RequestView
from frontend.const import SCOPES

if TYPE_CHECKING:
    from frontend.frontend import Frontend

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class LogoutView(RequestView):
    """View to handle logout requests."""

    requires_auth = False
    url = "/logout"

    async def get(self, frontend: "Frontend", request: web.Request):
        session = await aiohttp_session.get_session(request)
        session["username"] = None

        raise web.HTTPSeeOther(location="/")
