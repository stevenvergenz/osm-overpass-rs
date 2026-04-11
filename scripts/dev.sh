#!/bin/env bash
docker compose up -d
python3 -m http.server 8080 -d target/doc &> /dev/null &
P=$!
nvim .
kill $P
docker compose down
git commit --interactive -F -
git push
