import fileinput
import logging
import tempfile
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
        file_response = await frontend.core_client.request(
            method="GET",
            url="/api/file/" + entity_id,
        )

        resp = web.StreamResponse(status=200)
        resp.headers["Content-Type"] = "image/jpeg"
        resp.headers["Content-Length"] = str(len(file_response))
        await resp.prepare(request)
        await resp.write(file_response)
        return resp
