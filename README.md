# apis-saltans

![logo](logo.png)

A Rust library implementing a smart home protocol of dancing insects.

## Work in progress

This library aims to provide an implementation of the ZCL as defined in document `05-3474-23`, revision
`23.2`.

This library is a work in progress and is not yet ready for production use.

## Usage

This workspace contains multiple crates pertaining to the Zigbee protocol stack:

- [`aps`](./aps): The Zigbee APS layer implementation.
- [`zigbee-nwk`](./nwk): A Zigbee coordinator API using the actor model.
- [`zcl`](./zcl): The Zigbee Cluster Library implementation.
- [`zdp`](./zdp): The Zigbee Device Profile implementation.
- [`zigbee`](./zigbee): The Zigbee core protocol stack implementation.

## Legal

Tis library is free software and is not affiliated with the Zigbee Alliance or the Zigbee protocol.
It may or may not conform to the specifications linked to above.

## Contribution guidelines

* Format the code with `cargo +nightly fmt`.
* Check the code with `cargo clippy`.