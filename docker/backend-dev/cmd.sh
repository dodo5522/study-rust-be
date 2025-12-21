#!/bin/bash
set -euo pipefail

cargo-watch \
  --delay 1 \
  --watch Cargo.toml \
  --watch src \
  --watch layers \
  -x 'run'
