# wsl-aws-config

The purpose of this cli is to streamline the process of updating the aws credentials file.

In Windows you can run `aws configure` to update the file with new credentials.

This doesn't work in linux/WSL and as such you have to manually open the file and replace the contents with the new credentials.

# Installation

To install this app:
* clone this repository
* navigate to the repository root
* run `bash install.sh` - this will build a release version of the app and copy it to ~/.cargo/bin/
NOTE: if you do not have ~/.cargo/bin/ in your PATH you will need to add it, or copy the compiled output to a folder that is in your PATH, so you can run it anywhere with `wsl-aws-config`

# Running
* First, make sure you have your aws credentials copied to clipboard, you want to copy the whole credentials including the incorrect name ([xxxxxxxxxxxx_AWSAdministratorAccess]) as this will be replaced with [default] automatically
* Then, run `wsl-aws-config` in any terminal window.
* If successful you should see the config printed out in the console, if not check you have the right details copied and try again.
