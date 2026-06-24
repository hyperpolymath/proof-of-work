# SPDX-License-Identifier: MPL-2.0
FROM gcr.io/oss-fuzz-base/base-builder-rust

COPY . $SRC/proof-of-work
WORKDIR $SRC/proof-of-work

COPY .clusterfuzzlite/build.sh $SRC/
