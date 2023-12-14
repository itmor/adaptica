#!/bin/bash

# Цвета
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Запускаем компиляцию less
echo -e "${GREEN}Running gulp...${NC}"
gulp > /dev/null &

# Ждем 5 секунд
sleep 5

# Запускаем BrowserSync
echo -e "${GREEN}Starting BrowserSync...${NC}"
browser-sync start --config bs-config.js > /dev/null &

# Ждем еще 5 секунд
sleep 5

# Запускаем cargo watch
echo -e "${GREEN}Running cargo watch...${NC}"
cargo watch -s ./watch.sh
