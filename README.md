# apis-saltans

![logo](logo.png)

A Rust library implementing a smart home protocol of dancing insects.

## Work in progress

This library aims to provide an implementation of the ZCL as defined in document `05-3474-23`, revision
`23.2`.

This library is a work in progress and is not yet ready for production use.

## Usage

This workspace contains multiple crates pertaining to the Zigbee protocol stack:

- [`aps`](apis-saltans-aps): The Zigbee APS layer implementation.
- [`zcl`](apis-saltans-zcl): The Zigbee Cluster Library implementation.
- [`zdp`](apis-saltans-zdp): The Zigbee Device Profile implementation.
- [`zigbee`](apis-saltans-core): The Zigbee core protocol stack implementation.
- [`zigbee-coordinator`](apis-saltans-coordinator): A Zigbee coordinator API using the actor model.
- [`zigbee-hw`](apis-saltans-hw): A Zigbee hardware abstraction layer.

## Legal

This library is free software and is not affiliated with the Zigbee Alliance.
It may or may not conform to the official specifications of the Zigbee protocol.

## Contribution guidelines

* Format the code with `cargo +nightly fmt`.
* Check the code with `cargo clippy`.