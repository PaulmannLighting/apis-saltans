# TODOs

- [ ] Split APS and ZCL/ZDP layers.
- [ ] Add basic APS frame to incoming Zigbee events.
- [ ] Implement Zigbee event handler, which auto-responds to appropriate requests.
- [ ] Implement node discovery. See `ZigBeeNodeServiceDiscoverer`.
- [ ] Implement retrieving Node Descriptor from `NetworkManager`.
- [ ] Implement binding management.
- [ ] Implement persistent storage of nodes.

## Architectural challenges

- [ ] ZCL and ZDP frames are framed within APS frames, but e.g. EZSP does not allow the passing-in and retrieval of
  entire APS frames.