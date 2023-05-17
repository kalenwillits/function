#!/bin/bash

set -xe
cargo build --release
mkdir -p ~/Apps/fn/
cp target/release/fn ~/Apps/fn/fn
content="export PATH=\$PATH:~/Apps/fn/" 
if grep -Fxq $content ~/.bashrc 
then
echo $content >> ~/.bashrc
fi
echo "Insallation complete! Please restart your shell session..."
