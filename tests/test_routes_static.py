"""Test static application routes."""
from frontend.application import create_application


async def test_favicon(aiohttp_client):
    """Test favicon delivery."""

    app = create_application()

    client = await aiohttp_client(app)
    resp = await client.get("/static/favicon.ico")
    assert resp.status == 200
