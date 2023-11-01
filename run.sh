#!/bin/bash

cargo build --release > /dev/null 2>&1

cd wall-e-script

source .env/bin/activate
maturin develop > /dev/null 2>&1

cd ..

python3 "$1"
