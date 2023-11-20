# Rust Websockets Server

High Performance Websockets Server written in `Rust`.

```bash
     ✓ WebSocket connection established
     ✓ Status is 101

     checks................: 100.00%  ✓ 7354          ✗ 0
     data_received.........: 604 kB   2.0 kB/s
     data_sent.............: 2.5 GB   8.2 MB/s
     iteration_duration....: avg=4.82s min=2s      med=4.69s max=16.28s p(90)=7.69s p(95)=8.5s
     iterations............: 3677     12.188846/s
     vus...................: 1        min=1           max=100
     vus_max...............: 100      min=100         max=100
     ws_connecting.........: avg=2.3s  min=368.7µs med=2.16s max=14.27s p(90)=4.61s p(95)=5.38s
     ws_msgs_sent..........: 50183762 166353.590392/s
     ws_session_duration...: avg=3.82s min=1s      med=3.69s max=15.28s p(90)=6.69s p(95)=7.5s
     ws_sessions...........: 3677     12.188846/s

```

Includes a simple client for sending and receiving messages.

## Features

- Simple Design and Implementation.

- Broadcasts messages to all connected clients.

- Buffers messages in memory.

## Performance

- For an environment with `2GB` of Memory and `1VCPU` Available

  - `166,353.59` messages per Second Throughput
  - `100,727` connections established per Second

- Load Testing was done using the `k6` framework

## Running

```bash
# Running Locally
cargo run

# Deploying

# Build Binary
cargo build

```

## Production

- Deploy binaries to globally distributed servers.

- Use `Georouting` based off of `DNS` or `IP Header Information` to route to nearest Server for `Lowest Latency` and `Optimal Performance`

- Use existing Client code in `ws-client/` as a starting point for your WebSockets applications to build on top of.

## Docker

```bash
$ docker build -t ws-server-rs .
docker run -p 9001:9001 ws-server-rs
```
