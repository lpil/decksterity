#!/bin/sh

npm run webpack-dev-server &
WEBPACK=$!

npm run bsb-watch

kill $WEBPACK
