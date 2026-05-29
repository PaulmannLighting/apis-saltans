# TODOs

- [ ] Incorporate message back-channel into `NcpDriver` trait, so that drivers can send messages back to the
  coordinator.
- [ ] Add a method to `NcpDriver` to allow starting the actor and returning an `Ncp` proxy object.
- [ ] Allow to register event back-channels on the `Ncp` proxy object, so that the coordinator can receive events from
  the driver.
- [x] Implement `NcpDriver` trait as an external API for drivers to implement<s>, and for the coordinator to use</s>.