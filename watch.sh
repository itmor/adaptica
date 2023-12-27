#!/bin/bash
 
cargo run > >(while IFS= read -r line; do
  if [[ "$line" == *"Finished dev"* ]]; then
    echo -e "\e[32mCompile success\e[0m"
    curl -s http://localhost:3004/__browser_sync__?method=reload > /dev/null
  fi
done) 2>&1 &

sleep 5