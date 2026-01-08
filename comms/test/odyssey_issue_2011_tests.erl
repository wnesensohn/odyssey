-module(odyssey_issue_2011_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual(retry, odyssey_window:retry_budget(1, 3)).

boundary_test() -> ?assertEqual(exhausted, odyssey_window:retry_budget(3, 3)).

invalid_test() -> ?assertMatch({error, _}, odyssey_window:retry_budget(-1, 3)).
