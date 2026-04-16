-module(odyssey_issue_2101_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() ->
    {ok, B} = odyssey_frame:encode(1, 42, <<1, 2>>),
    ?assertMatch({ok, #{payload_bytes := 2}}, odyssey_frame:inspect(B)).
