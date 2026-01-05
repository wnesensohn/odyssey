# Engineering units

| Quantity | Representation | Unit |
| --- | --- | --- |
| Feed and tank pressure | Finite floating point | kPa |
| Temperature | Finite floating point | K |
| Battery energy | Finite floating point | Wh |
| Bus demand and heater power | Finite floating point | W |
| Throttle and state of charge | Closed interval [0,1] | Fraction |
| Position | Three finite components | m |
| Velocity | Three finite components | m/s |
| Torque | Three finite components | N m |
| Rate | Finite floating point | rad/s |
| Clock and sample timestamps | Unsigned integer | ms |

The UI performs presentation formatting only. Conversion at a subsystem boundary
must be explicit. No controller accepts NaN or infinity as a missing-value marker.
Invalid measurements carry quality metadata and fail the relevant interlock.
