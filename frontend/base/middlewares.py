import logging
import time
from typing import Any, Awaitable, Callable, Dict
from xml.dom.expatbuilder import parseString

import aiohttp_jinja2
import aiohttp_session
from aiohttp import hdrs, web
from frontend import i18n
from frontend.login.handlers import oauth_client, save_token_pair_locally, threadLocalData

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


@web.middleware
async def auth_token(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
) -> web.StreamResponse:
    """checks and refresh token if close to expiration."""

    expires_in = threadLocalData.expires_in
    refresh_token = threadLocalData.refresh_token

    # if expiry and refresh token is available
    if expires_in is not None and refresh_token is not None:
        now = int(time.time())

        # refresh token if it expires soon
        if (expires_in - now) <= 180:
            access_token, provider_data = await core_client.get_access_token(refresh_token, grant_type="refresh_token")
            save_token_pair_locally(access_token, provider_data)

    return await handler(request)


@web.middleware
async def i18n_middleware(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
) -> web.StreamResponse:
    """Change i18n based on language header."""
    locale = request.headers.get("Accept-Language")
    i18n.setLocale(locale=locale)

    return await handler(request)


@web.middleware
async def login_required_middleware(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
) -> web.StreamResponse:
    """Validates user authentication from current session."""
    require_login = getattr(handler, "__require_login__", False)
    session = await aiohttp_session.get_session(request)
    username = session.get("username")
    _LOGGER.debug("session username: " + str(username))

    # redirect if not logged-in
    if require_login:
        _LOGGER.debug("require_login")
        if not username:
            _LOGGER.debug("not username")
            raise web.HTTPUnauthorized()

    return await handler(request)


@web.middleware
async def error_middleware(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
):
    status = 500
    try:
        response = await handler(request)
        if response.status != 404:
            return response
        message = response.message
        status = response.status
    except web.HTTPException as ex:
        if ex.status != 404:
            raise
        message = ex.reason
        status = ex.status
    return aiohttp_jinja2.render_template(
        "error.jinja2", request, {"status": status, "error": str(message)}, status=400
    )


async def handle_401(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
):
    response = aiohttp_jinja2.render_template(
        "login_required.jinja2",
        request,
        {
            "request": request,
            "status_code": 401,
        },
    )
    response.set_status(401)
    return response


async def handle_404(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
):
    response = aiohttp_jinja2.render_template("error.jinja2", request, {"request": request, "status_code": 404})
    response.set_status(404)
    return response


async def handle_500(
    request: web.Request,
    handler: Callable[[web.Request], Awaitable[web.StreamResponse]],
):
    _LOGGER.warn(request)
    response = aiohttp_jinja2.render_template("error.jinja2", request, {"request": request, "status_code": 500})
    response.set_status(500)
    return response


def error_pages(overrides):
    async def middleware(app, handler):
        async def middleware_handler(request):
            try:
                response = await handler(request)
                override = overrides.get(response.status)
                if override is None:
                    return response
                else:
                    return await override(request, response)
            except web.HTTPException as ex:
                override = overrides.get(ex.status)
                if override is None:
                    raise
                else:
                    return await override(request, ex)

        return middleware_handler

    return middleware


error_middleware = error_pages(
    {
        401: handle_401,
        404: handle_404,
        500: handle_500,
    }
)
