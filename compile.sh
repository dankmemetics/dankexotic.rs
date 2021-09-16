#!/bin/bash

set -v
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=program