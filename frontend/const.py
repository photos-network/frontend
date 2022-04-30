"""Constants used by Photos.network frontend."""
MAJOR_VERSION = 0
MINOR_VERSION = 1
PATCH_VERSION = 1
__short_version__ = f"{MAJOR_VERSION}.{MINOR_VERSION}"
__version__ = f"{__short_version__}.{PATCH_VERSION}"
REQUIRED_PYTHON_VER = (3, 7, 1)

FRONTEND_VERSION = __version__

KEY_AUTHENTICATED = "authenticated"
KEY_USER_ID = "user_id"

SCOPES = "openid profile email phone library:read library:write"
