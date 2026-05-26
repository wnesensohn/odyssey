-module(odyssey_wide_1_tests).
-include_lib("eunit/include/eunit.hrl").
wide_test() ->
    {ok, B} = odyssey_frame:encode(3, 4294967295, <<>>),
    ?assertEqual(12, byte_size(B)).
