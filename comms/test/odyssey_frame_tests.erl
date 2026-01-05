-module(odyssey_frame_tests).
-include_lib("eunit/include/eunit.hrl").

crc_reference_test() -> ?assertEqual(16#29b1, odyssey_frame:crc16(<<"123456789">>)).

round_trip_test_() ->
    [
        ?_test(begin
            Payload = binary:copy(<<16#5a>>, Length),
            {ok, Frame} = odyssey_frame:encode(2, 42, Payload),
            ?assertEqual(
                {ok, #{kind => 2, sequence => 42, payload => Payload}}, odyssey_frame:decode(Frame)
            )
        end)
     || Length <- [0, 1, 16, 1024]
    ].

corrupt_and_truncated_test() ->
    {ok, Frame} = odyssey_frame:encode(1, 100, <<1, 2, 3>>),
    lists:foreach(
        fun(Length) ->
            ?assertMatch({error, _}, odyssey_frame:decode(binary:part(Frame, 0, Length)))
        end,
        lists:seq(0, byte_size(Frame) - 1)
    ),
    <<First, Tail/binary>> = Frame,
    ?assertMatch({error, _}, odyssey_frame:decode(<<(First bxor 1), Tail/binary>>)).

stream_reassembly_test() ->
    {ok, First} = odyssey_frame:encode(1, 1, <<10>>),
    {ok, Second} = odyssey_frame:encode(1, 2, <<20>>),
    Partial = binary:part(Second, 0, 7),
    ?assertEqual(
        {ok, [First], Partial}, odyssey_frame:split_stream(<<First/binary, Partial/binary>>)
    ),
    ?assertEqual(
        {ok, [First, Second], <<>>}, odyssey_frame:split_stream(<<First/binary, Second/binary>>)
    ).
