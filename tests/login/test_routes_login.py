"""Test login routes."""
from typing import Generator

import aiohttp_session
from aiohttp import web
from aiohttp.test_utils import TestClient
from aiohttp_session import get_session, new_session
from frontend.application import create_application


async def test_login_redirect(aiohttp_client: Generator):
    """Test login redirect."""
    # given
    app = create_application()
    client: TestClient = await aiohttp_client(app)

    # when
    resp = await client.get("/login", allow_redirects=False)

    # then
    assert resp.status == 302


async def test_logout_endpoint(aiohttp_client: Generator):
    """Test if logout endpoint is valid."""
    # given
    app = create_application()
    client: TestClient = await aiohttp_client(app)

    # when
    resp = await client.get("/logout")

    # then
    assert resp.status == 200


async def test_logout_redirect(aiohttp_client: Generator):
    """Test if logout clears session."""
    # given
    app = create_application()
    client: TestClient = await aiohttp_client(app)
    await client.get("/logout")

    # when
    resp = await client.get("/settings", allow_redirects=False)

    # then
    assert resp.status == 401
