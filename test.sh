#!/usr/bin/env bash
set -e

cd ./korvin-core

wasm-pack test  --firefox --headless -- . "${@}"
