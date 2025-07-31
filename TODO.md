# Design changes

## Clusters and commands

- [ ] Make clusters an enum.
- [ ] Make commands enums.
- [ ] Make respective command enums inner types of cluster variants.

## Robustness

- [ ] Split read and write attributes into separate enums.
    - [ ] Enforce correct types on write and allow lax input on read.