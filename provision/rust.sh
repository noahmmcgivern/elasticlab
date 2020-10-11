#!/bin/bash

curl --proto '=https' --tlsv1.2 -sSf -o rust.sh https://sh.rustup.rs
bash rust.sh -y
rm rust.sh