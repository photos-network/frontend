import fileinput
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


class PhotoView(RequestView):
    """View to handle logout requests."""

    requires_auth = False
    url = "/photo/{entity_id}"

    async def get(
        self,
        frontend: "Frontend",
        request: web.Request,
        entity_id: str,
    ):
        file = await frontend.oauth_client.request(
            method="GET",
            url="/api/file/" + entity_id,
        )

        _LOGGER.debug("+++++++")
        _LOGGER.debug(type(file))
        _LOGGER.debug(len(file))

        return web.StreamResponse(body=file, status=200)
