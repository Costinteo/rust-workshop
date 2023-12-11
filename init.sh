#!/usr/bin/env bash

# Prepare for embedded build
# Only quick solution I could find

git submodule update --init
cp -r ./embassy-workshop ./embassy

echo "Copied $PWD/embassy-workshop to $PWD/embassy/embassy-workshop"
