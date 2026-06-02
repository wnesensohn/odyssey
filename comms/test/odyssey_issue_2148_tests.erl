-module(odyssey_issue_2148_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual(ready, odyssey_session:pending_state(1, 16)).

boundary_test() -> ?assertEqual(pressured, odyssey_session:pending_state(12, 16)).

invalid_test() -> ?assertEqual(full, odyssey_session:pending_state(16, 16)).
