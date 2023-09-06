#!/usr/bin/env bash
cargo b -r
sudo rm  /bin/adduce
sudo cp target/release/adduce /bin/
echo "Completed."
