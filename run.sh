#!/usr/bin/env bash
set -eoxu pipefail

cargo run --bin generate -- hypercosm.hidl json/hypercosm.json
cargo run --bin document --  hypercosm.hidl proto-docs
mdbook build