-module(odyssey_issue_2070_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual(accept, odyssey_gateway:admit_worker(3, 4)).

boundary_test() -> ?assertEqual(backpressure, odyssey_gateway:admit_worker(4, 4)).

invalid_test() -> ?assertMatch({error, _}, odyssey_gateway:admit_worker(-1, 4)).
