-module(odyssey_issue_2193_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertMatch(#{wire_version := 4}, odyssey_frame:capabilities(4)).
