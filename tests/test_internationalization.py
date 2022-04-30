"""Test application Internationalization."""
from frontend.application import create_application


async def test_login_button_in_de(aiohttp_client):
    """Test Internationalized login button in header layout."""

    app = create_application()

    client = await aiohttp_client(app)
    resp = await client.get("/", headers={"Accept-Language": "de"})
    assert resp.status == 200
    text = await resp.text()
    assert "Anmeldung" in text


async def test_login_button_in_en(aiohttp_client):
    """Test Internationalized login button in header layout."""

    app = create_application()

    client = await aiohttp_client(app)
    resp = await client.get("/", headers={"Accept-Language": "en"})
    assert resp.status == 200
    text = await resp.text()
    assert "Login" in text
