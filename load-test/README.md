# k6

## Running Load Test

- `k6 run ws-load-test.js`

## Setup

- `brew install k6`

```bash

docker build -t kalz_websocket_server .

docker run -p 9001:9001 kalz_websocket_server

# Test Websockets

brew install websocat

websocat ws://localhost:9001


```
