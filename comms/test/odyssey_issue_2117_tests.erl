-module(odyssey_issue_2117_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual({ok, 50}, odyssey_window:ack_latency(100, 150)).

boundary_test() -> ?assertEqual({ok, 0}, odyssey_window:ack_latency(100, 100)).

invalid_test() -> ?assertMatch({error, _}, odyssey_window:ack_latency(150, 100)).
