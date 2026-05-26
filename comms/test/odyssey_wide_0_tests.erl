-module(odyssey_wide_0_tests).
-include_lib("eunit/include/eunit.hrl").
wide_test() ->
    {ok, B} = odyssey_frame:encode(1, 70000, <<1, 2>>),
    ?assertMatch({ok, #{sequence := 70000}}, odyssey_frame:decode(B)).
