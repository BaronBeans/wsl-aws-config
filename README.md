# aws-config-wsl

The purpose of this cli is to streamline the process of updating the aws credentials file.

In Windows you can run `aws configure` to update the file with new credentials.

This doesn't work in linux/WSL and as such you have to manually open the file and replace the contents with the new credentials.

# Installation

To install rust if you don't already have it:
* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

To install this app:
* clone this repository
* navigate to the repository root
* run `bash install.sh` - this will build a release version of the app and copy it to ~/.cargo/bin/

NOTE: if you do not have ~/.cargo/bin/ in your PATH you will need to add it, or copy the compiled output to a folder that is in your PATH, so you can run it anywhere with `aws-config-wsl`

# Running
* First make sure you are connected to the pulse vpn.
* Run the powershell script ./vault-login.ps1 to log into aws and write the credentials needed to ~/.aws/vault.txt.
* Run aws-config-wsl to copy the relevant config from ~/.aws/vault.txt to ~/.aws/credentials in the correct format.
