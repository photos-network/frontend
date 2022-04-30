import logging
import time
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


class OauthCallbackView(RequestView):
    """View to handle oauth callback after login."""

    requires_auth = False
    url = "/callback"

    async def get(self, frontend: "Frontend", request: web.Request):
        """Initiate the authorization code grant flow and redirect the user ."""

        # its not a valid redirect, if 'code' is not within the query
        if frontend.oauth_client.shared_key not in request.url.query:
            _LOGGER.warn("invalid redirect call! ")

            # redirect to oauth provider
            raise web.HTTPUnauthorized()

        code = request.url.query["code"]
        state = frontend.oauth_client.state
        _LOGGER.warn("code: " + str(code) + " => " + str(state))
        _LOGGER.warn("redirect_uri: " + str(frontend.config.redirect_uri))

        # request a token pair from the authorization server
        access_token, provider_data = await frontend.oauth_client.get_access_token(
            code=code,
            redirect_uri=frontend.config.redirect_uri,
            state=state,
        )

        # calculate timestamp
        timestamp = time.time()
        expires_in = int(timestamp) + provider_data["expires_in"]

        # collect informations for current user
        user, data = await frontend.oauth_client.user_info()

        # save data into session
        session = await aiohttp_session.get_session(request)
        session["username"] = str(user.username)
        session["first_name"] = str(user.first_name)
        session["last_name"] = str(user.last_name)
        session["expires_in"] = expires_in
        session["access_token"] = str(provider_data["access_token"])
        session["refresh_token"] = str(provider_data["refresh_token"])

        _LOGGER.debug("session " + str(session))

        raise web.HTTPSeeOther(location="/")
