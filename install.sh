#!/usr/bin/env bash
set -e

echo "Building Adduce..."
cargo build --release

echo "Installing Adduce..."
if [[ -f /bin/adduce ]]; then
    sudo rm /bin/adduce
fi

doas cp target/release/adduce ~/.local/bin/
echo "Completed!"
