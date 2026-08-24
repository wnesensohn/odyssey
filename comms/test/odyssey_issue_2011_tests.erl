-module(odyssey_issue_2011_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual(retry, odyssey_window:retry_budget(1, 3)).

boundary_test() -> ?assertEqual(exhausted, odyssey_window:retry_budget(3, 3)).

invalid_test() -> ?assertMatch({error, _}, odyssey_window:retry_budget(-1, 3)).

final_attempt_is_reserved_for_ack_test() ->
    ?assertEqual(exhausted, odyssey_window:retry_budget(2, 3)).

retry_delay_nominal_test() -> ?assertEqual({ok, 2010}, odyssey_window:retry_delay(500, 2, 10)).
