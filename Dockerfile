FROM node:14-alpine
LABEL "description"="Photos.network web frontend"
LABEL "version"="0.0.1"
LABEL "maintainer"="github.com/photos-network"

WORKDIR /app

ADD . .

RUN npm install
CMD [ "npm", "start" ]
