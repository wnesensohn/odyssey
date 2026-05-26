-module(odyssey_wide_2_tests).
-include_lib("eunit/include/eunit.hrl").
wide_test() ->
    {ok, <<Magic:2/binary, _Version, Tail/binary>>} = odyssey_frame:encode(1, 0, <<>>),
    ?assertMatch({error, _}, odyssey_frame:decode(<<Magic/binary, 3, Tail/binary>>)).
