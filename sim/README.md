# Deterministic flight scenarios

| Input | Purpose |
| --- | --- |
| nominal.csv | Feed readiness, ignition and shutdown progression |
| pump-overcurrent.csv | Pump current trip during startup |

Times are mission-relative milliseconds. Pressure values use kPa, temperature
uses K, and current uses A. The input rows are observations, not actuator commands.
A current trip requires operator review before propulsion is rearmed.
