"""Entry point of Photos.network frontend."""

import asyncio
import logging
import os
import sys
import time
from typing import Any, Awaitable, Callable, Dict
from xml.dom.expatbuilder import parseString

import aiohttp_jinja2
import aiohttp_session
import jinja2
from aioauth_client import OAuth2Client
from aiohttp import ClientSession, web

import i18n
from frontend.const import REQUIRED_PYTHON_VER

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


# TODO: use from configuration http://172.22.5.1:7777/callback
# redirect_uri = "http://172.22.11.226:7778/callback"


# async def token_updated(new_token: str):
#     print("new token " + str(new_token))


# oauth_session = OAuth2Session(
#     client_id="d37c098d-ac25-4a96-b462-c1ca05f45952",
#     client=None,
#     auto_refresh_url="https://photos.stuermer.pro/api/oauth/token",
#     auto_refresh_kwargs=None,
#     scope="openid profile email phone library:read library:write",
#     redirect_uri="https://photos.stuermer.pro/callback",
#     token=None,
#     state=None,
#     token_updater=token_updated,
# )

# class CoreOauth2Client(OAuth2Client):
#     """Oauth2 client for Photos.network core instance."""

#     access_token_url = 'https://photos.stuermer.pro/api/oauth/token'
#     authorize_url = 'https://photos.stuermer.pro/api/oauth/authorize'
#     base_url = 'https://photos.stuermer.pro'
#     name = 'Photos.network Core'
#     user_info_url = 'https://photos.stuermer.pro/api/user/'

#     @staticmethod
#     def user_parse(data: Dict[str, Any]):
#         """Parse information from the provider."""
#         yield 'id', data.get('id')
#         yield 'username', data.get('email')
#         yield 'first_name', data.get("firstname")
#         yield 'last_name', data.get("lastname")

# oauth_client = CoreOauth2Client(
#     client_id = "d37c098d-ac25-4a96-b462-c1ca05f45952",
#     client_secret = "AYgD5Y2DV7bbWupYW7WmYQ",
#     logger = _LOGGER
# )


# def validate_python() -> None:
#     """Validate that the right Python version is running."""
#     if sys.version_info[:3] < REQUIRED_PYTHON_VER:
#         print(
#             "Photos.network requires at least Python "
#             f"{REQUIRED_PYTHON_VER[0]}.{REQUIRED_PYTHON_VER[1]}.{REQUIRED_PYTHON_VER[2]}"
#         )
#         sys.exit(1)


# async def username_ctx_processor(request: web.Request) -> Dict[str, Any]:
#     """Jinja2 context processor to extract the username from an active session."""
#     session = await aiohttp_session.get_session(request)
#     username = session.get("username")

#     return {"username": username}


# async def get_files_for_user(request: web.Request) -> Dict[str, Any]:
#     """Load files where the current user has access"""
#     session = await aiohttp_session.get_session(request)

#     client_session = ClientSession()

#     auth_header = "Bearer " + session["access_token"]
#     headers = {"Authorization": auth_header}
#     resp = await client_session.get(url="https://photos.stuermer.pro/api/photos?limit=10&offset=0", headers=headers)
#     if resp.status == 200:
#         print(resp.status)
#         data = await resp.json()
#         print(data)
#         print(data["results"])

#         return data["results"]
#     return []


# def main() -> int:
#     """Start Photos.network application."""
#     validate_python()

#     app = web.Application(client_max_size=64 * 1024**2)
#     aiohttp_session.setup(app, aiohttp_session.SimpleCookieStorage())
#     env = aiohttp_jinja2.setup(
#         app,
#         loader=jinja2.FileSystemLoader("frontend/views"),
#         extensions=["jinja2.ext.i18n", "jinja2.ext.debug"],
#         context_processors=[username_ctx_processor],
#     )
#     env.install_gettext_translations(i18n, newstyle=True)
#     # app.middlewares.append(auth_token)
#     app.middlewares.append(check_login)
#     app.middlewares.append(error_middleware)

#     # @aiohttp_jinja2.template("index.jinja2")
#     # async def index(request: web.Request) -> Dict[str, Any]:
#     #     locale = request.headers.get("Accept-Language")
#     #     i18n.setLocale(locale)

#     #     files = []

#     #     session = await aiohttp_session.get_session(request)

#     #     if "username" in session:
#     #         files = await get_files_for_user(request)

#     #     return {"files": files}

#     @aiohttp_jinja2.template("edit.html")
#     async def edit_photo(request: web.Request) -> Dict[str, Any]:
#         post_id = request.match_info["post"]
#         db = request.config_dict["DB"]
#         # return {"post": await fetch_post(db, post_id)}
#         return {"photo": None}

#     async def edit_photo_apply(request: web.Request) -> web.Response:
#         post_id = request.match_info["post"]
#         db = request.config_dict["DB"]
#         post = await request.post()
#         await db.execute(
#             "UPDATE posts SET title = ?, text = ? WHERE id = ?",
#             [post["title"], post["text"], post_id],
#         )
#         await db.commit()
#         raise web.HTTPSeeOther(location=f"/{post_id}/edit")

#     # app.router.add_static("/static", os.path.join(os.getcwd(), "frontend/static"))
#     # app.router.add_get("/", index, name="index")
#     # app.router.add_get("/login", login, name="login")
#     # app.router.add_get("/logout", logout, name="logout")
#     app.router.add_post("/{photo}/edit", edit_photo, name="edit_photo")
#     app.router.add_post("/photo/edit", edit_photo_apply, name="edit_photo_apply")
#     # app.router.add_get("/callback", oauth_callback, name="oauth_callback")

#     web.run_app(app)


# if __name__ == "__main__":
#     sys.exit(main())
