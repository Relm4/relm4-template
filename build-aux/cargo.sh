#!/bin/bash

export MESON_BUILD_ROOT="$1"
export MESON_SOURCE_ROOT="$2"
export CARGO_TARGET_DIR="$MESON_BUILD_ROOT"/target
export CARGO_HOME="$CARGO_TARGET_DIR"/cargo-home
export SHORTWAVE_LOCALEDIR="$4"

if [[ $5 = "Devel" ]]
then
    echo "DEBUG MODE"
    cargo build --manifest-path \
        "$MESON_SOURCE_ROOT"/Cargo.toml --verbose && \
        cp "$CARGO_TARGET_DIR"/debug/gtk-rust-template $3
else
    echo "RELEASE MODE"
    cargo build --manifest-path \
        "$MESON_SOURCE_ROOT"/Cargo.toml --release && \
        cp "$CARGO_TARGET_DIR"/release/gtk-rust-template $3
fi

