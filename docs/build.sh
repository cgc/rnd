#!/bin/bash

OUT=docs
mkdir $OUT

# Gather parcel projects for building
PARCEL_DIRS=$(
    git ls-files {,**/}package.json |
    while read line; do
        if grep parcel $line --quiet; then
            echo $(dirname $line)
        fi
    done
)

# Copy all git files over.
git ls-files . | while read line; do
    mkdir -p $(dirname $OUT/$line)
    cp $line $OUT/$line
done

# Build all parcel projects
echo "$PARCEL_DIRS" | while read line; do
    rm -r ./$line/dist/*
    cd $line
    npx parcel build --public-url /rnd/$line
    cd -

    rm -r ./$OUT/$line/*
    cp -r $line/dist/ $OUT/$line
done

