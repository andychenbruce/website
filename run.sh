#!/usr/bin/env bash
set -e

rm -fr root_server
cargo run --bin "static_site_generator" -- --config-path "site_src/config.json"
cargo run --bin "andy_http_server" -- --port 1234 --root-serve-path root_server
