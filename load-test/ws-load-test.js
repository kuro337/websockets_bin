import ws from "k6/ws";
import { check, sleep } from "k6";

export let options = {
  stages: [
    { duration: "4s", target: 10 } /* ramp up to 10 connections   */,
    { duration: "1m", target: 10 } /* stay at 10 connections      */,
    { duration: "1m", target: 100 } /* ramp up to 100 connections  */,
    { duration: "2m", target: 100 } /* stay at 100 connections     */,
    { duration: "30s", target: 0 } /* stay at 100 connections     */,
  ],
};

export default function () {
  const url = "ws://127.0.0.1:9001";
  const params = { tags: { my_tag: "hello" } };

  let totalMessagesSent = 0;
  let totalMessagesReceived = 0;

  const res = ws.connect(url, params, function (socket) {
    check(socket, {
      "WebSocket connection established": (s) => s !== null,
    });

    socket.on("open", function open() {
      console.log("connected");
      socket.send(JSON.stringify({ type: "name", user_name: "test" }));

      let messageCount = 0;
      const startTime = new Date().getTime();

      while (new Date().getTime() - startTime < 1000) {
        socket.send(
          JSON.stringify({ type: "message", content: "Test message" })
        );
        messageCount++;
      }

      socket.close();

      const messagesPerSecond = messageCount / 1;
      console.log(`Messages per second: ${messagesPerSecond}`);
    });

    socket.on("message", function (message) {
      console.log(`Received message: ${message}`);
      check(message, {
        "Received the expected message": (msg) => msg.indexOf("Message") !== -1,
      });
    });

    socket.on("close", function () {
      console.log("disconnected");
    });

    socket.on("error", function (e) {
      if (e.error() !== "websocket: close sent") {
        console.log("An unexpected error occurred: ", e.error());
      }
    });

    socket.setTimeout(function () {
      console.log("11 seconds passed, closing the socket");
      socket.close();
    }, 11000);
  });

  check(res, { "Status is 101": (r) => r && r.status === 101 });
  sleep(1);
}
