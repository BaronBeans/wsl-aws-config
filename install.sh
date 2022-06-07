#!/bin/sh
cargo build --release
sudo cp ./target/release/wsl-aws-config ~/.cargo/bin/wsl-aws-config
