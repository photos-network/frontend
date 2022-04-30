"""Base class for webserver requests."""
import asyncio
import json
import logging
from typing import TYPE_CHECKING, Any, Callable, List, Optional

from aiohttp import web
from aiohttp.typedefs import LooseHeaders
from aiohttp.web_exceptions import HTTPInternalServerError, HTTPUnauthorized
from frontend.const import KEY_AUTHENTICATED, KEY_USER_ID

if TYPE_CHECKING:
    from frontend.frontend import Frontend

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


def is_callback(func: Callable[..., Any]) -> bool:
    """Check if function is safe to be called in the event loop."""
    return getattr(func, "_callback", False) is True


class RequestView:
    """Base request."""

    requires_auth = True
    url: Optional[str] = None

    extra_urls: List[str] = []

    @staticmethod
    def json(
        result: Any,
        status_code: int = 200,
        headers: Optional[LooseHeaders] = None,
    ) -> web.Response:
        """Return a JSON response."""
        try:
            msg = json.dumps(result, cls=ComplexEncoder, allow_nan=False).encode("UTF-8")
        except (ValueError, TypeError) as err:
            _LOGGER.error(f"Unable to serialize to JSON: {err}\n{result}")
            raise HTTPInternalServerError from err
        response = web.Response(
            body=msg,
            content_type="application/json",
            status=status_code,
            headers=headers,
        )
        response.enable_compression()
        return response

    def json_message(
        self,
        message: str,
        status_code: int = 200,
        message_code: Optional[str] = None,
        headers: Optional[LooseHeaders] = None,
    ) -> web.Response:
        """Return a JSON message response."""
        data = {"message": message}
        if message_code is not None:
            data["code"] = message_code
        return self.json(data, status_code, headers=headers)

    def register(self, frontend: "Frontend", router: web.UrlDispatcher) -> None:
        """Register the view with a router."""
        assert self.url is not None, "No url set for view"
        urls = [self.url] + self.extra_urls
        routes = []

        for method in ("get", "post", "delete", "put", "patch", "head", "options"):
            handler = getattr(self, method, None)

            if not handler:
                continue

            handler = request_handler_factory(self, frontend, handler)

            for url in urls:
                routes.append(router.add_route(method, url, handler))


class ComplexEncoder(json.JSONEncoder):
    """Encoder for complex classes."""

    def default(self, o):
        """Encode all properties."""
        if isinstance(o, complex):
            return [o.real, o.imag]
        # Let the base class default method raise the TypeError.
        return json.JSONEncoder.default(self, o)


def request_handler_factory(view: RequestView, frontend: "Frontend", handler: Callable) -> Callable:
    """Wrap the handler classes."""
    assert asyncio.iscoroutinefunction(handler) or is_callback(handler), "Handler should be a coroutine or a callback."

    async def handle(request: web.Request) -> web.StreamResponse:
        """Handle incoming request."""
        if frontend.is_stopping:
            return web.Response(status=503)

        authenticated = request.get(KEY_AUTHENTICATED, False)

        if view.requires_auth and not authenticated:
            raise HTTPUnauthorized()

        _LOGGER.info(f"Serving {request.path} to {request.remote} (auth: {authenticated})")
        _LOGGER.info(f"match_info {request.match_info}")

        # try:
        result = await handler(frontend, request, **request.match_info)

        if asyncio.iscoroutine(result):
            result = await result
        # except voluptuous.Invalid as err:
        #     raise HTTPBadRequest() from err
        # except exceptions.ServiceNotFound as err:
        #     raise HTTPInternalServerError() from err
        # except exceptions.Unauthorized as err:
        #     raise HTTPUnauthorized() from err

        if isinstance(result, web.StreamResponse):
            # The method handler returned a ready-made Response, how nice of it
            return result

        status_code = 200

        if isinstance(result, tuple):
            result, status_code = result

        bresult = convert_to_bytes(result)

        return web.Response(body=bresult, status=status_code)

    return handle


def convert_to_bytes(input: Any) -> bytes:
    """Convert given input into bytes."""
    if isinstance(input, bytes):
        bresult = input
    elif isinstance(input, str):
        bresult = input.encode("utf-8")
    elif input is None:
        bresult = b""
    else:
        assert False, f"Result should be None, string, bytes or Response. Got: {input}"

    return bresult
