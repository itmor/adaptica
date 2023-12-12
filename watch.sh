#!/bin/bash

cargo run > >(while IFS= read -r line; do
  echo "$line"

  if [[ "$line" == *"Finished dev"* ]]; then
    echo -e "\e[32mReload Browser\e[0m"
    curl -s http://localhost:3003/__browser_sync__?method=reload > /dev/null
  fi
done) 2>&1 &

sleep 5