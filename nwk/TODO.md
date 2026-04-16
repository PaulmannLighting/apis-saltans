# TODOs

- [ ] Continue working on demultiplexer.
- [x] Split APS and ZCL/ZDP layers.
- [x] Add basic APS frame to incoming Zigbee events.
- [ ] Implement Zigbee event handler, which auto-responds to appropriate requests.
- [ ] Implement node discovery. See `ZigBeeNodeServiceDiscoverer`.
- [ ] Implement retrieving Node Descriptor from `NetworkManager`.
- [ ] Implement binding management.
- [ ] <s>Implement persistent storage of nodes.</s> _Not here, but in a separate crate._

## Architectural challenges

- [x] ZCL and ZDP frames are framed within APS frames, but e.g. EZSP does not allow the passing-in and retrieval of
  entire APS frames and only processes and provides part of the APS metadata.
- [x] Remove `zigbee_nwk::aps` in favor of `aps::Data` frame.
- [ ] <s>Introduce APS transport layer underneath ZCL and ZDP layers which will be used to send those frames.</s> _Use
  APS instead_