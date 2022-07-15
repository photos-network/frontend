import asyncio
import logging
import time
from dataclasses import dataclass
from typing import Any, Dict

import aiohttp
import aiohttp_session
from aiohttp import request, web
from frontend.config import Config

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class CoreClient:
    """Http client for Photos.network core communication.

    * Handles oauth flow https://developers.photos.network/core/authentication_flow/
    """

    state: str = None
    accessToken: str = None
    refreshToken: str = None
    expiresIn: int = None
    scope: str = None

    def __init__(self, config: Config) -> None:
        self.config: Config = config

    def get_authorize_url(self, scope: str, state: str = None) -> str:
        """generate authorize url based on inputs."""
        self.scope = scope

        return (
            str(self.config.core_url)
            + ":"
            + str(self.config.core_port)
            + "/api/oauth/authorize?client_id="
            + str(self.config.client_id)
            + "&response_type=code&redirect_uri="
            + str(self.config.redirect_uri)
            + "&response_mode=query&scope="
            + scope
            + "&state="
            + state
        )

    async def get_access_token(self, code: str):
        """request access token."""
        url = str(self.config.core_url) + ":" + str(self.config.core_port) + "/api/oauth/token"

        raw_data = (
            "grant_type=authorization_code&code="
            + code
            + "&client_id="
            + self.config.client_id
            + "&redirect_uri="
            + self.config.redirect_uri
        )

        async with aiohttp.ClientSession() as session:
            async with session.request(
                method="POST",
                url=url,
                headers={"Content-Type": "application/x-www-form-urlencoded"},
                data=raw_data,
                ssl=None,
                verify_ssl=False,
            ) as resp:
                response = await resp.json()

                self.accessToken = response["access_token"]
                self.refreshToken = response["refresh_token"]
                self.expiresIn = response["expires_in"]

                await session.close()

                return {
                    "expires_in": response["expires_in"],
                    "access_token": response["access_token"],
                    "refresh_token": response["refresh_token"],
                }

    async def check_access_token(self) -> int:
        """check if current access token is still valid."""
        url = str(self.config.core_url) + ":" + str(self.config.core_port) + "/api/protected"

        async with aiohttp.ClientSession() as session:
            async with session.get(url=url, headers={"Authorization": "Bearer " + str(self.accessToken)}) as resp:
                return resp.status

    async def user_info(self):
        """user info."""

        url = str(self.config.core_url) + ":" + str(self.config.core_port) + "/api/user/"

        async with aiohttp.ClientSession() as session:
            async with session.get(url=url, headers={"Authorization": "Bearer " + str(self.accessToken)}) as resp:
                response = await resp.json()

                return UserInfo(
                    response["id"],
                    response["email"],
                    response["firstname"],
                    response["lastname"],
                )

    async def refresh_access_token_call(self):
        """refresh access token"""

        url = str(self.config.core_url) + ":" + str(self.config.core_port) + "/api/oauth/token"

        if self.refreshToken is not None and self.scope is not None:

            raw_data = (
                "grant_type=refresh_token"
                + "&refresh_token="
                + str(self.refreshToken)
                + "&client_id="
                + self.config.client_id
                + "&scope="
                + self.scope
            )

            async with aiohttp.ClientSession() as clientSession:
                async with clientSession.request(
                    method="POST",
                    url=url,
                    headers={"Content-Type": "application/x-www-form-urlencoded"},
                    data=raw_data,
                    ssl=None,
                    verify_ssl=False,
                ) as resp:
                    response = await resp.json()

                    timestamp = time.time()
                    expires_in = int(timestamp) + response["expires_in"]

                    self.accessToken = response["access_token"]
                    self.refreshToken = response["refresh_token"]
                    self.expiresIn = expires_in

                    status = resp.status
                    await clientSession.close()

                    return (status, self.accessToken, self.refreshToken, self.expiresIn)
        else:
            return (None, None, None, None)

    async def get_photos(self):
        async with aiohttp.ClientSession() as session:
            async with session.get(
                url=str(self.config.core_url) + ":" + str(self.config.core_port) + "/api/photos",
                headers={"Authorization": "Bearer " + str(self.accessToken)},
            ) as resp:
                return await resp.json()

    async def request(self, method: str, url: str):
        """Check if user is authenticated and communicate with core instance."""

        async with aiohttp.ClientSession() as session:
            async with session.request(
                method=method,
                url=str(self.config.core_url) + ":" + str(self.config.core_port) + url,
                headers={"Authorization": "Bearer " + str(self.accessToken)},
            ) as resp:
                return await resp.read()


@dataclass
class UserInfo:
    id: str
    username: str
    first_name: str
    last_name: str

    def __init__(self, id: str, username: str, first_name: str, last_name: str):
        self.id = id
        self.username = username
        self.first_name = first_name
        self.last_name = last_name
