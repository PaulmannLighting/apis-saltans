# TODOs

- [x] Implement APS layer actor.
- [ ] Translate APS sequence to `messageTag` in EZSP.
- [x] Make the response channel for acknowledgements for outgoing APS frames optional, depending on the
  `TxOptions::ACKNOWLEDGED_TRANSMISSION` flag.
- [ ] In EZSP, respect `TxOptions::FRAGMENTATION_PERMITTED` and fail on too-large frames.
