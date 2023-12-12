how to turn on hot reloading on actix

run in root
`$ browser-sync start --config bs-config.js`
enter you server host and port in config, enter actix server
`proxy: "127.0.0.2:8080" `

run cargo watch -s ./watch.sh

open http://localhost:3003/



when troubleshooting errors with a busy port call
`$ lsof -i :8080` 
output
`
COMMAND     PID USER   FD   TYPE   DEVICE SIZE/OFF NODE NAME
adaptica 223735 alt    9u  IPv4 33214522      0t0  TCP sert-HP-ProBook-450-G5:http-alt (LISTEN)
`

end kill process width output pid
`kill 223735`