#!/bin/sh
cargo build --release
sudo cp ./target/release/aws-config-wsl ~/.cargo/bin/aws-config-wsl
