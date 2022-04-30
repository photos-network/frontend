from typing import Any, Awaitable, Callable, Dict

from aiohttp import web


def require_login(
    func: Callable[[web.Request], Awaitable[web.StreamResponse]],
) -> Callable[[web.Request], Awaitable[web.StreamResponse]]:
    """Decorator to indicate internal calls checked by the `login_required_middleware`."""
    func.__require_login__ = True  # type: ignore
    return func
