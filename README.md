# wsl-aws-config

The purpose of this cli is to streamline the process of updating the aws credentials file.

In Windows you can run `aws configure` to update the file with new credentials.

This doesn't work in linux/WSL and as such you have to manually open the file and replace the contents with the new credentials.

# Installation

To install this app:
* clone this repository
* navigate to the repository root
* run `cargo build --release`
* copy the file `/target/build/release/wsl-aws-config` to somewhere in your PATH, so you can run it
