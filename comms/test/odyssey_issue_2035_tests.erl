-module(odyssey_issue_2035_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assert(odyssey_heartbeat:recovery_ready([degraded, connected, connected], 2)).

boundary_test() -> ?assertNot(odyssey_heartbeat:recovery_ready([connected, degraded], 2)).

invalid_test() -> ?assertNot(odyssey_heartbeat:recovery_ready([], 0)).
