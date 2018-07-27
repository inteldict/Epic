#!/bin/sh

PROJECT=$(basename `pwd`)

SERVER="kunzd@linux.uni-koblenz.de:~/public_html/"

echo $PROJECT
echo $SERVER

cargo build --release
scp target/release/${PROJECT} ${SERVER}${PROJECT}.cgi

