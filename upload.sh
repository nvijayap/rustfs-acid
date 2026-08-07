#!/usr/bin/env bash

# upload.sh - uploads a file

echo; date > file.txt

printf "Content of file.txt (to be uploaded): `cat file.txt`\n"

./run.sh upload file.txt

aws --endpoint http://jisnukrsna.world:9000 s3 cp s3://ey0/file.txt .

printf "\nContent of uploaded and retrieved file.txt: `cat file.txt`\n\n"
