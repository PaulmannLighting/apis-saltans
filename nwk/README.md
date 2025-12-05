# Zigbee Network management

## Documentation

This crate provides traits to implement a hardware-agnostic Zigbee Network management layer.

Therefor it uses the [actor model](https://en.wikipedia.org/wiki/Actor_model), providing an `Actor` trait for
coordinator object and a `Proxy` trait to communicate with the actor.

The `Actor` trait is auto-implemented for any type implementing the `Nlme` (network layer management entity) trait.

Therefore, it is sufficient to implement the `Nlme` trait for your hardware-specific Zigbee coordinator type.

## Example

The following pseudo-code demonstrates how to use the crate with a hypothetical

```rust
use tokio::spawn;
use tokio::sync::mpsc::channel;
use zigbee_nwk::{Actor, Nlme, Proxy};

use my_hardware_zigbee_coordinator::MyZigbeeCoordinator;

impl Nlme for MyZigbeeCoordinator {
    // implement required methods...
}


#[tokio::main]
async fn main() {
    // Initialize your hardware-specific Zigbee coordinator
    let coordinator = MyZigbeeCoordinator::new(/* ... */);

    // Create a communication channel between the proxy and the actor.
    let (proxy, actor) = channel(1024);

    // Spawn the actor in a separate task.
    spawn(coordinator.run(actor));

    // Use the proxy to send Zigbee commands and receive responses.
    match proxy.unicast(/* ... */).await {
        Ok(response) => {
            println!("Received response: {response:?}");
        }
        Err(error) => {
            eprintln!("Error sending command: {error:?}");
        }
    }
}
```