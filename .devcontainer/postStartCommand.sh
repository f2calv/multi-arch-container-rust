#!/bin/sh

echo "postStartCommand.sh"
echo "-------------------"

sudo apt-get update
sudo apt-get upgrade -y

rustup --version
rustc --version

# alias cls="clear"
# alias cc="cargo check"
# alias cb="cargo build"
# alias cr="cargo run"

echo "Done"