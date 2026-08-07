#!/usr/bin/env bash

# upload.sh - uploads a file

echo; date > file.txt

printf "Content of file.txt (to be uploaded): `cat file.txt`\n"

./run.sh upload file.txt

ENDPOINT=`awk -F= '/^DISTRIBUTED_STORAGE_URL/ {print $2}' .env`
printf "ENDPOINT: $ENDPOINT, "

BUCKET=`awk -F= '/^BUCKET/ {print $2}' .env`
printf "BUCKET: $BUCKET\n\n"

aws --endpoint $ENDPOINT s3 cp s3://$BUCKET/file.txt .

printf "\nContent of uploaded and retrieved file.txt: `cat file.txt`\n\n"
