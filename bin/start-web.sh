#!/bin/sh

npm run bsb-watch &
BSB=$!

npm run webpack-dev-server &
WEBPACk=$!

while read line ; do
  :
done

kill $BSB
kill $WEBPACk
