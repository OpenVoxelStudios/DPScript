#!/bin/bash

DIR="$(dirname "$(realpath "$0")")"

function compile {
    TARGET="$1"

    if [[ "$(rustup show)" != *"$TARGET"* ]]; then
        rustup target add "$TARGET"
    fi

    cargo build --release --bin dscls --target "$TARGET" --manifest-path "$DIR/../Cargo.toml"

    mkdir -p "$DIR/bin/$TARGET"
    cp "$DIR/../target/$TARGET/release/dscls" "$DIR/bin/$TARGET/dscls"
}

compile "x86_64-unknown-linux-gnu"
