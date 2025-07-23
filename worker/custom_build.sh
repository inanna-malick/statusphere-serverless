#!/bin/bash
# NOTE: this exists only so cloudflare build (via the build pipeline integration) works
set -e

# check if cargo is installed
if ! hash cargo 2>&1 >/dev/null
then
    echo "cargo not installed. We're probably in CI, so let's fix that now"
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    . "$HOME/.cargo/env"
fi
cargo install -q worker-build && worker-build --release