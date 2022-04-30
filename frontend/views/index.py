import logging
from typing import TYPE_CHECKING, Any, Awaitable, Callable, Dict

import aiohttp_jinja2
import aiohttp_session
from aiohttp import web
from frontend.base.request import RequestView
from frontend.const import KEY_AUTHENTICATED

if TYPE_CHECKING:
    from frontend.frontend import Frontend

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class IndexView(RequestView):
    """View to handle Status requests."""

    requires_auth = False
    url = "/"

    async def get(self, frontend: "Frontend", request: web.Request):
        """Retrieve photos grid or landing page if not authenticated."""

        files = []

        authenticated = request.get(KEY_AUTHENTICATED, False)
        if authenticated:
            files = await self._get_files_for_user(frontend, request)

        response = aiohttp_jinja2.render_template(
            template_name="index.jinja2",
            request=request,
            context={"files": files},
        )

        return response

    async def head(self, frontend: "Frontend", request: web.Request):
        """Retrieve if frontend is running."""
        return self.json_message("Frontend is running.")

    async def _get_files_for_user(self, frontend: "Frontend", request: web.Request) -> Dict[str, Any]:
        """Load files where the current user has access"""
        session = await aiohttp_session.get_session(request)

        user = await frontend.oauth_client.request(
            method="GET",
            url="/api/photos?limit=10&offset=0",
        )

        _LOGGER.debug(user)

        if "results" in user:
            return user["results"]

        return []
