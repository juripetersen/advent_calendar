FROM node:14

WORKDIR /var/www/html

RUN apt-get update -y --fix-missing

RUN npm install -g @vue/cli --unsafe-perm

RUN npm set strict-ssl false

RUN npm run serve
