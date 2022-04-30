import logging
from typing import Any, Dict

from aioauth_client import OAuth2Client
from frontend.config import Config


class CoreOauth2Client(OAuth2Client):
    """Oauth2 client for Photos.network core instance.

    * Flow: https://developers.photos.network/core/authentication_flow/
    """

    state: str = None

    def set_from_config(
        self,
        config: Config,
        logger: logging.Logger = None,
    ) -> None:
        self.name = "Photos.network Core"
        self.base_url = str(config.core_url) + ":" + str(config.core_port)
        self.access_token_url = self.base_url + "/api/oauth/token"
        self.authorize_url = self.base_url + "/api/oauth/authorize"
        self.user_info_url = self.base_url + "/api/user/"
        self.client_id = str(config.client_id)
        self.client_secret = str(config.client_secret)
        self.logger = logger

    @staticmethod
    def user_parse(data: Dict[str, Any]):
        """Parse user information."""
        yield "id", data.get("id")
        yield "username", data.get("email")
        yield "first_name", data.get("firstname")
        yield "last_name", data.get("lastname")
