# Implement clusters

- [x] Measurement and Sensing
    - [x] Attributes
    - [ ] Split attributes into /R/W/P

# Design

- [x] Define a strategy for handling reporting of attributes from different clusters.
- [ ] Implement serialization of `read_attributes::Response`.
- [ ] Revisit Power Configuration attributes.
- [ ] Remove `MANUFACTURER_CODE` from `Command` trait and make it runtime-changeable.
- [ ] Remove `DISABLE_DEFAULT_RESPONSE` from `Command` trait and make it runtime-changeable.