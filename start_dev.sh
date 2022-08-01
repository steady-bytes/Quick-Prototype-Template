#!/bin/sh

# start back-end
docker-compose up -d

# start front-end
npm start --prefix ./front_end
