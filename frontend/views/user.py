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

        user = await frontend.core_client.user_info()

        _LOGGER.debug(str(user))

        response = aiohttp_jinja2.render_template(
            template_name="index/user.jinja2",
            request=request,
            context={"username": username, "first_name": user.first_name, "last_name": user.last_name},
        )

        return response

    async def post(self, frontend: "Frontend", request: web.Request):
        """update user profile and preferences."""

        _LOGGER.warn("apply user changes is not implemented yet")

        data = await request.post()
        _LOGGER.error(str(data))

        if "first-name" in data:
            first_name = data["first-name"]
            _LOGGER.debug("new first-name: " + str(first_name))

        if "last-name" in data:
            last_name = data["last-name"]
            _LOGGER.debug("new last-name: " + str(last_name))

        if "about" in data:
            about = data["about"]
            _LOGGER.debug("new about: " + str(about))

        if "password" in data:
            _LOGGER.debug("new password: " + str(data["password"]))

        user = await frontend.core_client.user_info()

        _LOGGER.debug("user: " + str(user))

        user_id = user.id
        update_url = "/api/user/" + str(user_id)

        raw_data = '{"firstname": "' + str(first_name) + '", "lastname": "Administrator"}'
        _LOGGER.debug("payload: " + str(raw_data))
        _LOGGER.debug("payload: " + str(type(raw_data)))

        # TODO: patch user

        # await frontend.core_client.patch_user(
        #     method="PATCH",
        #     url=update_url,
        #     headers={"Content-Type": "application/x-www-form-urlencoded"},
        #     data=raw_data,
        # )

        raise web.HTTPFound(location="/user")
