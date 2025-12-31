# SPDX-License-Identifier: AGPL-3.0-or-later
FROM gcr.io/oss-fuzz-base/base-builder-rust

COPY . $SRC/proof-of-work
WORKDIR $SRC/proof-of-work

COPY .clusterfuzzlite/build.sh $SRC/
