#!/bin/sh

set -euo pipefail

cargo build --target x86_64-alpine-linux-musl $*
# strip all symbols from executables to generate smaller sizes
find target/x86_64-alpine-linux-musl/ -type f -executable \
    -maxdepth 2 -mindepth 1 -exec strip -s '{}' \;