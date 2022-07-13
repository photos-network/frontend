FROM python:3.10
LABEL "description"="Photos.network web frontend"
LABEL "version"="0.2.1"
LABEL "maintainer"="github.com/photos-network"

WORKDIR /app

ADD . .

RUN python3 setup.py install

CMD [ "python3", "/usr/local/bin/frontend" ]
