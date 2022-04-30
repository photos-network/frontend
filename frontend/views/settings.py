from typing import TYPE_CHECKING

import aiohttp_jinja2
from aiohttp import web
from frontend.base.request import RequestView

if TYPE_CHECKING:
    from frontend.frontend import Frontend


class SettingsView(RequestView):
    """View to handle user settings."""

    requires_auth = True
    url = "/settings"

    async def get(self, frontend: "Frontend", request: web.Request):
        response = aiohttp_jinja2.render_template(
            template_name="settings.jinja2",
            request=request,
            context={},
        )

        return response
