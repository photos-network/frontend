"""Photos.network setup script"""
import sys

from setuptools import setup

from frontend import const

if sys.version_info < (3, 0):
    print("{PROJECT_NAME} requires python version >= 3.0")
    sys.exit(1)

setup(
    name="frontend",
    version=const.FRONTEND_VERSION,
    description="The default web frontend for photos.network",
    long_description="The default web frontend for photos.network to manage components.",
    author="The Photos.network Authors",
    author_email="devs@photos.network",
    url="https://developers.photos.network/frontend/",
    license="Apache License 2.0",
    classifiers=[
        "Intended Audience :: End Users/Desktop",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Scientific/Engineering :: Atmospheric Science",
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.10",
    ],
    keywords=["docker", "photos-network", "api"],
    zip_safe=False,
    platforms="any",
    packages=[
        "frontend",
    ],
    entry_points={"console_scripts": ["frontend = server:main"]},
    include_package_data=True,
    package_data={
        "frontend": ["addons/**/*.py", "addons/**/model/**", "addons/**/dto/**"],
    },
    install_requires=[
        "python-i18n>=0.3.9",
        "aiohttp-devtools>=1.0",
        "aiohttp-jinja2>=1.4.2",
        "aiohttp_session>=2.11.0",
        "aioauth-client>=0.27.0",
    ],
)
