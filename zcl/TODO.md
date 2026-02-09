# Implement clusters

- [x] OnOff
    - [x] Commands
        - [x] Off
        - [x] On
        - [x] Toggle
        - [x] OffWithEffect
        - [x] OnWithRecallGlobalScene
        - [x] OnWithTimedOff
    - [x] Attributes
- [x] LevelControl
    - [x] Commands
        - [x] MoveToLevel
        - [x] Move
        - [x] Step
        - [x] Stop
        - [x] MoveToLevelWithOnOff
        - [x] MoveWithOnOff
        - [x] StepWithOnOff
        - [x] StopWithOnOff
    - [x] Attributes
- [x] Alarms
    - [x] Commands
        - [x] ResetAlarm
        - [x] ResetAllAlarms
        - [x] GetAlarm
        - [x] ResetAlarmLog
        - [x] Alarm
        - [x] GetAlarmResponse
    - [x] Attributes
- [x] Time
    - [x] Attributes
- [x] Identify
    - [x] Commands
    - [x] Attributes

# Design

- [ ] Define a strategy for handling reporting of attributes from different clusters.
- [ ] Implement serialization of `read_attributes::Response`.