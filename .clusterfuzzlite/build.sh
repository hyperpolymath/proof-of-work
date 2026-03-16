#!/bin/bash -eu
# SPDX-License-Identifier: PMPL-1.0-or-later

cd $SRC/proof-of-work

# Build fuzz targets
cargo +nightly fuzz build

# Copy fuzz targets
for target in $(cargo +nightly fuzz list); do
    cp fuzz/target/x86_64-unknown-linux-gnu/release/$target $OUT/
done
