#!/bin/env bash

# test for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "Error: uncommitted changes"
    exit 1
fi

# test for compilation
cargo rustc --lib -- -W missing-docs -D warnings
if [ $? -gt 0 ]; then
    exit 1
fi

# test for formatting
cargo fmt
if [ -n "$(git status --porcelain)" ]; then
    echo "Error: code is not properly formatted"
    exit 1
fi
