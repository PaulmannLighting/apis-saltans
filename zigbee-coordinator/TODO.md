# TODOs

- [ ] Implement the Zigbee coordinator layer as scribbled
  under https://excalidraw.com/#json=R8eYU4Ih_4V9bA8V-nlid,bR_cHhjqULNxlNLH64QoLg
- [x] Implement a thread pool for the actors to prevent DOS when many devices join messages are incoming.
- [x] Parallelize discovery steps by communicating with the transceivers in separate tasks, then sending messages of the
  results back via the loopback to the actor.
- [x] Differentiate between Zigbee responses and (hardware-) events, and handle them accordingly.