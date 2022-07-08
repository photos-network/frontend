"""Configurations for frontend instance."""
import logging
from typing import Optional

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Config:
    """Representation class of configurations."""

    def __init__(
        self,
        frontend_url: str = "http://127.0.0.1",
        frontend_port: int = 7778,
        core_url: str = "http://127.0.0.1",
        core_port: int = 7777,
        client_id: str | None = None,
        client_secret: str | None = None,
        redirect_uri: str | None = None,
        cookie_name: str = "Photos.network",
        cookie_secret: str = "",
    ) -> None:
        self.frontend_url: Optional[str] = frontend_url
        self.frontend_port: int = frontend_port
        self.core_url: Optional[str] = core_url
        self.core_port: int = core_port
        self.client_id: str = client_id
        self.client_secret: str = client_secret
        self.redirect_uri: str = redirect_uri
        self.cookie_name: str = cookie_name
        self.cookie_secret: str = cookie_secret

    @classmethod
    def fromConfigFile(cls, conf_dict: dict):
        """create config instance from configuration file."""

        config = cls()

        if "frontend_url" in conf_dict:
            config.frontend_url = conf_dict["frontend_url"]

        if "frontend_port" in conf_dict:
            config.frontend_port = conf_dict["frontend_port"]

        if "core_url" in conf_dict:
            config.core_url = conf_dict["core_url"]

        if "core_port" in conf_dict:
            config.core_port = conf_dict["core_port"]

        if "client_id" in conf_dict:
            config.client_id = conf_dict["client_id"]

        if "client_secret" in conf_dict:
            config.client_secret = conf_dict["client_secret"]

        if "redirect_uri" in conf_dict:
            config.redirect_uri = conf_dict["redirect_uri"]

        if "cookie_name" in conf_dict:
            config.cookie_name = conf_dict["cookie_name"]

        if "cookie_secret" in conf_dict:
            config.cookie_secret = conf_dict["cookie_secret"]

        return config
