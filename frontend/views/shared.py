import fileinput
import logging
from hashlib import sha1
from random import SystemRandom
from typing import TYPE_CHECKING

import aiohttp_jinja2
import aiohttp_session
from aiohttp import web
from frontend.base.request import RequestView
from frontend.const import SCOPES

if TYPE_CHECKING:
    from frontend.frontend import Frontend

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class SharedView(RequestView):
    """View to handle shared links."""

    requires_auth = False
    url = "/shared/{entity_id}"

    async def get(
        self,
        frontend: "Frontend",
        request: web.Request,
        entity_id: str,
    ):
        _LOGGER.warn("/shared/" + entity_id)

        session = await aiohttp_session.get_session(request)
        username = session.get("username")

        # TODO: get files for shared album
        files = []

        response = aiohttp_jinja2.render_template(
            template_name="index/shared.jinja2",
            request=request,
            context={"files": files},
        )

        return response
