# apis-saltans-hw

An abstraction layer for Zigbee hardware.

## Usage

This library provides a hardware abstraction layer to implement drivers for Zigbee hardware.
Therefor the library exports two main traits to be implemented by the hardware library:

### NcpDriver

The `NcpDriver` trait is used to implement a driver for a Zigbee network co-processor (NCP).
It provides methods to send and receive Zigbee messages as well as some other methods to manage the Zigbee network

### EventTranslator

The `EventTranslator` trait is used to implement a translator for Zigbee events.
It is to be used to translate hardware-specific events into a common Zigbee `Event` data structure.

### `Ncp`

This library provides an `Ncp` trait, which is implemented for any handle (sender) to actors that implement the
`NcpDriver` trait.
This trait is used on the [coordinator layer](../apis-saltans-coordinator) to send Zigbee messages to the NCP.

Each `Ncp::unicast` call represents one unicast frame to one short ID. The
hardware abstraction layer does not perform parallel unicast fan-out; callers
that need to target multiple devices must issue multiple unicast requests or use
Zigbee multicast/broadcast operations where appropriate.
