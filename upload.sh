#!/usr/bin/env bash

# upload.sh

echo; aws s3 ls s3://ey0/file.txt --endpoint http://jisnukrsna.world:9000; ./run.sh upload file.txt; aws s3 ls s3://ey0/file.txt --endpoint http://jisnukrsna.world:9000; echo
