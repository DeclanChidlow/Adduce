#!/usr/bin/env bash
set -e

cargo build

if [[ -f /bin/adduce ]]; then
    sudo rm /bin/adduce
fi

sudo cp target/release/adduce /bin/
echo "Completed!"