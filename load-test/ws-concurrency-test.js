import ws from "k6/ws";
import { check, sleep } from "k6";

export let options = {
  iterations: 1,
  vus: 1,
};

const connectionCounts = [10]; // Numbers of connections to test
const url = "ws://127.0.0.1:9001";
const params = { tags: { my_tag: "hello" } };

export default function () {
  for (const connections of connectionCounts) {
    const start = new Date();
    const sockets = [];

    for (let i = 0; i < connections; i++) {
      const socket = ws.connect(url, params, function (socket) {
        check(socket, {
          "WebSocket connection established": (s) => s.readyState === 1,
        });
      });

      sockets.push(socket);
    }

    check(sockets[0], { "Status is 101": (r) => r && r.status === 101 });
    // sleep(1);

    const end = new Date();
    const elapsedTime = (end - start) / 1000;
    console.log(
      `Established ${connections} connections in ${elapsedTime.toFixed(
        3
      )} seconds`
    );

    sockets.forEach((socket) => socket.close());
  }
}
