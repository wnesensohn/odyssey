-module(odyssey_gateway_tests).
-include_lib("eunit/include/eunit.hrl").

tcp_gateway_round_trip_test() ->
    {ok, Pid, Port} = odyssey_gateway:start(0, fun(#{payload := Payload}) -> Payload end),
    try
        {ok, Socket} = gen_tcp:connect({127, 0, 0, 1}, Port, [binary, {active, false}], 1000),
        {ok, Frame} = odyssey_frame:encode(1, 42, <<"nominal">>),
        ok = gen_tcp:send(Socket, Frame),
        {ok, Bytes} = gen_tcp:recv(Socket, 17, 1000),
        ?assertEqual(
            {ok, #{kind => 2, sequence => 42, payload => <<"nominal">>}},
            odyssey_frame:decode(Bytes)
        ),
        gen_tcp:close(Socket)
    after
        odyssey_gateway:stop(Pid)
    end.
