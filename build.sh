#!/bin/bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run
cargo build --release

