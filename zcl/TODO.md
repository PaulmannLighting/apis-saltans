# Implement clusters

- [x] Implement attribute `Id` types.
    - [ ] Establish design for nested enums.
- [ ] <s>Implement a `ClusterId` enum and use it in the `Cluster` trait.</s>
- [x] Measurement and Sensing
    - [x] Attributes
    - [x] Split attributes into /R/W/P

# Design

- [x] Define a strategy for handling reporting of attributes from different clusters.
- [ ] <s>Implement serialization of `read_attributes::Response`.</s>
- [x] <s>Revisit Power Configuration attributes.</s>
- [x] Move `MANUFACTURER_CODE` from the core `Cluster` trait into ZCL command and attribute traits.
