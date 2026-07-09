# apis-saltans

![logo](logo.png)

A Rust library implementing a smart home protocol of dancing insects.

## Work in progress

This library aims to provide an implementation of the ZCL as defined in document `05-3474-23`, revision
`23.2`.

This library is a work in progress and is not yet ready for production use.

## Usage

This workspace contains multiple crates pertaining to the Zigbee protocol stack:

- [`apis-saltans-aps`](apis-saltans-aps): The Zigbee APS layer implementation.
- [`apis-saltans-nwk`](apis-saltans-nwk): Transport-neutral Zigbee NWK source, destination, metadata, and envelope types.
- [`apis-saltans-zcl`](apis-saltans-zcl): The Zigbee Cluster Library implementation.
- [`apis-saltans-zdp`](apis-saltans-zdp): The Zigbee Device Profile implementation.
- [`apis-saltans-core`](apis-saltans-core): The Zigbee core protocol stack implementation.
- [`apis-saltans-coordinator`](apis-saltans-coordinator): A Zigbee coordinator API using the actor model.
- [`apis-saltans-hw`](apis-saltans-hw): A Zigbee hardware abstraction layer.

## Legal

This library is free software and is not affiliated with the Zigbee Alliance.
It may or may not conform to the official specifications of the Zigbee protocol.

## Contribution guidelines

* Format the code with `cargo +nightly fmt`.
* Check the code with `cargo clippy`.
