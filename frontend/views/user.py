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


class UserView(RequestView):
    """View to handle logout requests."""

    requires_auth = True
    url = "/user"

    async def get(self, frontend: "Frontend", request: web.Request):
        """Display user profile and preferences."""
        session = await aiohttp_session.get_session(request)
        username = session.get("username")

        user = await frontend.oauth_client.request(
            method="GET",
            url="/api/user/",
        )

        _LOGGER.debug(user)

        last_name = ""
        if "lastname" in user:
            last_name = user["lastname"]

        first_name = ""
        if "firstname" in user:
            first_name = user["firstname"]

        response = aiohttp_jinja2.render_template(
            template_name="index/user.jinja2",
            request=request,
            context={"username": username, "first_name": first_name, "last_name": last_name},
        )

        return response

    async def post(self, frontend: "Frontend", request: web.Request):
        """update user profile and preferences."""

        _LOGGER.warn("apply user changes is not implemented yet")

        data = await request.post()
        first_name = data["first-name"]
        _LOGGER.debug("new first-name: " + str(first_name))

        user = await frontend.oauth_client.request(
            method="GET",
            url="/api/user/",
        )

        _LOGGER.debug("user: " + str(user))

        user_id = user["id"]
        update_url = "/api/user/" + str(user_id)

        raw_data = '{"firstname": "' + str(first_name) + '", "lastname": "Administrator"}'
        _LOGGER.debug("payload: " + str(raw_data))
        _LOGGER.debug("payload: " + str(type(raw_data)))

        _LOGGER.debug("access_token: " + str(frontend.oauth_client.access_token))

        await frontend.oauth_client.request(
            method="PATCH",
            url=update_url,
            headers={"Content-Type": "application/x-www-form-urlencoded"},
            data=raw_data,
        )

        raise web.HTTPFound(location="/user")
