#!/usr/bin/env bash
cargo b -r
sudo rm  /usr/bin/adduce
sudo cp ../target/release/adduce /usr/bin/
echo "Completed."
