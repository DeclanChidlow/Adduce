#!/usr/bin/env bash
set -e

echo "Building Adduce..."
cargo build

echo "Installing Adduce..."
if [[ -f /bin/adduce ]]; then
    sudo rm /bin/adduce
fi

sudo cp target/release/adduce /bin/
echo "Completed!"