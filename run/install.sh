#!/bin/bash

set -xe;
cargo build --release;
mkdir -p ~/Apps/fn/;
cp target/release/fn ~/Apps/fn/fn;
echo "export PATH=\$PATH:~/Apps/fn/" >> ~/.bashrc;
echo "Insallation complete! Please restart your shell session";
