#!/bin/bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup update

cargo install trunk

rustup target add wasm32-unknown-unknown

