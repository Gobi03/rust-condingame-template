#!/bin/bash

set -eu

cargo build --release
# cargo check

cd tool/make-submit
./make-submit-file.py

cd ../..
pbcopy < src/submit.rs
