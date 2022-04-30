from aiohttp import web
from aiohttp_session import get_session


class Handler(web.View):
    async def get_current_user(self) -> str | None:
        """Current user"""

        session = await get_session(self.request)
        email = session.get("email", None)

        if email is None:
            return None

        return email
