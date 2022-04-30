from aiohttp import web
from frontend.base.oauth import CoreOauth2Client
from frontend.config import Config


class CoreClient:
    """Http client for Photos.network core communication.

    * Handles oauth flow https://developers.photos.network/core/authentication_flow/
    """

    oauth_client: CoreOauth2Client = None

    def __init__(self, config: Config) -> None:
        pass

    async def refresh_token():
        """refresh access token"""
        pass

    async def request(
        self,
        request: web.Request,
    ):
        """Check if user is authenticated and communicate with core instance."""
        authenticated = False
        # TODO: validate auth
        # TODO: refresh auth
        pass
