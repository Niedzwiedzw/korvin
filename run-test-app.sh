#!/usr/bin/env bash
set -e

cd ./test-app
trunk serve --port 8090 --watch ../ --ignore ../target "$@"
