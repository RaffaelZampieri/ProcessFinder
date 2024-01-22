#!/bin/bash

if [-f "target/release/pf"]; then
    :
    else 
        cargo build --release
fi

sudo chmod +x target/release/pf

sudo cp target/release/pf /usr/local/bin