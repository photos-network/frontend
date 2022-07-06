import logging
from hashlib import sha1
from random import SystemRandom
from typing import TYPE_CHECKING

from aiohttp import web
from frontend.base.request import RequestView
from frontend.const import SCOPES

if TYPE_CHECKING:
    from frontend.frontend import Frontend

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class LoginView(RequestView):
    """View to handle login requests."""

    requires_auth = False
    url = "/login"

    async def get(self, frontend: "Frontend", request: web.Request):
        """Initiate the authorization code grant flow and redirect the user ."""
        # create an opaque value to prevent cross-site requests
        state = str(sha1(str(SystemRandom().random()).encode("ascii")).hexdigest())

        frontend.core_client.state = state

        authorization_url = frontend.core_client.get_authorize_url(
            scope=SCOPES,
            state=state,
        )

        # redirect the user-agent to the generated authorization endpoint
        raise web.HTTPFound(location=authorization_url)
