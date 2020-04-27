#!/usr/bin/env bash

URL="https://t0guvf0w17.execute-api.us-east-1.amazonaws.com/Prod"

if [ $# -le 1 ] 
then 
    echo 'Usage: addref.sh REFERENCES_FILE QUERY'
    exit 1
fi 

QUERY=$2
DESTINATION=$1

SEARCH=$(curl -s -d $QUERY -H 'Content-Type: text/plain' $URL/web)
BIBTEX=$(curl -s --fail --data "$SEARCH" -H 'Content-Type: application/json' "$URL/export?format=bibtex")

if [ $? -ne 0 ] 
then
    echo 'Error when searching for query.'
    exit 1
fi

echo Adding "$BIBTEX"

echo '%' $(date +%Y-%m-%dT%H:%M:%S%z) from $QUERY >> $DESTINATION
echo "$BIBTEX" >> $DESTINATION
echo >> $DESTINATION

