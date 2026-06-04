# TODOs

- [ ] <s>Incorporate message back-channel into `NcpDriver` trait, so that drivers can send messages back to the
  coordinator.</s>
- [x] Add a method to `NcpDriver` to allow starting the actor and returning an `Ncp` proxy object.
- [ ] <s>Allow to register event back-channels on the `Ncp` proxy object, so that the coordinator can receive events
  from
  the driver.</s>
- [x] Implement `NcpDriver` trait as an external API for drivers to implement<s>, and for the coordinator to use</s>.