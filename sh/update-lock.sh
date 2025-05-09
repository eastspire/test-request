#!/bin/bash
cd ./test-request;
cargo build --release;
cd ../web-server;
cargo build --release;
cd ../;
