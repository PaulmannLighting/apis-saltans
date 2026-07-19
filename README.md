# apis-saltans

![logo](logo.png)

A Rust library implementing a smart home protocol of dancing insects.

## Work in progress

This library aims to provide an implementation of the ZCL as defined in document `05-3474-23`, revision
`23.2`.

This library is a work in progress and is not yet ready for production use.

## Usage

This workspace contains multiple crates pertaining to the Zigbee protocol stack:

- [`apis-saltans-aps`](aps): The Zigbee APS layer implementation.
- [`apis-saltans-nwk`](nwk): Transport-neutral Zigbee NWK source, destination, metadata, and envelope types.
- [`apis-saltans-zcl`](zcl): The Zigbee Cluster Library implementation.
- [`apis-saltans-zdp`](zdp): The Zigbee Device Profile implementation.
- [`apis-saltans-core`](core): The Zigbee core protocol stack implementation.
- [`apis-saltans-coordinator`](coordinator): A Zigbee coordinator API using the actor model.
- [`apis-saltans-hw`](hw): A Zigbee hardware abstraction layer.

Public failure types implement Rust's standard `Error` trait. Errors that retain a lower-level
failure expose it through `Error::source` and support `From` conversion, so applications can use
the `?` operator without discarding the original cause. Channel errors represented only as a
closed-send or closed-receive condition intentionally discard the channel payload.

## Legal

This library is free software and is not affiliated with the Zigbee Alliance.
It may or may not conform to the official specifications of the Zigbee protocol.

## Contribution guidelines

* Format the code with `cargo +nightly fmt`.
* Check the code with `cargo clippy`.
