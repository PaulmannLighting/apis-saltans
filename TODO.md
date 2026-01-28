# TODOs

## Implementation

- [ ] Introduce `Options` to cover `options_mask` and `options_override` and manually implement `FromLeStream` for it to
  make it optional.
- [ ] Implement `Attribute` for `power_configuration::attribute::write`.
- [ ] Implement all ZCL frames.
- [ ] Implement all Device Profile Client Services (binding, etc.) in ZDP.

## Design changes

N/A

## Robustness

- [ ] Split read and write attributes into separate enums.
- [ ] Enforce correct types on write and allow lax input on read.

## Correctness

- [ ] Enforce range constraints on attributes when writing.