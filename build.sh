#!/bin/bash


if [ ! -e ./_test ]; then
    mkdir ./_test
fi

cargo build --release && cp ./target/release/saba ./_test
