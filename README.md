# wsl-aws-config

The purpose of this cli is to streamline the process of updating the aws credentials file.

In Windows you can run `aws configure` to update the file with new credentials.

This doesn't work in linux/WSL and as such you have to manually open the file and replace the contents with the new credentials.

# Installation

To install this app:
* clone this repository
* navigate to the repository rootcargo build --release
* run `bash install.sh` - this will build a release version of the app and copy it to ~/.cargo/bin/
