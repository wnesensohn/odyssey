-module(odyssey_state_tests).
-include_lib("eunit/include/eunit.hrl").

queue_capacity_test() ->
    Empty = odyssey_queue:new(1),
    {ok, Full} = odyssey_queue:push(Empty, command),
    ?assertEqual({error, queue_full}, odyssey_queue:push(Full, another)),
    {ok, command, Next} = odyssey_queue:pop(Full),
    ?assertEqual(0, odyssey_queue:size(Next)),
    ?assertEqual({empty, Next}, odyssey_queue:pop(Next)).

window_ack_and_retry_test() ->
    {ok, Frame} = odyssey_frame:encode(1, 42, <<10>>),
    {ok, Window} = odyssey_window:reserve(odyssey_window:new(2, 100), 1000, Frame),
    ?assertEqual({error, window_full}, odyssey_window:reserve(Window, 1000, Frame)),
    {[], Before} = odyssey_window:due(Window, 1099),
    {[Frame], Retried} = odyssey_window:due(Before, 1100),
    {true, Acknowledged} = odyssey_window:acknowledge(Retried, 42),
    ?assertEqual(0, odyssey_window:size(Acknowledged)).

heartbeat_transitions_test() ->
    Empty = odyssey_heartbeat:new(100),
    ?assertEqual(disconnected, odyssey_heartbeat:state(Empty, 0)),
    {ok, Heartbeat} = odyssey_heartbeat:observe(Empty, 1000),
    ?assertEqual(connected, odyssey_heartbeat:state(Heartbeat, 1100)),
    ?assertEqual(degraded, odyssey_heartbeat:state(Heartbeat, 1150)),
    ?assertEqual(disconnected, odyssey_heartbeat:state(Heartbeat, 1201)),
    ?assertEqual({error, time_reversed}, odyssey_heartbeat:observe(Heartbeat, 999)).

session_lifecycle_test() ->
    {ok, Pid} = odyssey_session:start_link(#{sink => self()}),
    {ok, Sequence} = odyssey_session:send(Pid, 1, <<10>>),
    receive
        {downlink, Frame} -> ?assertMatch({ok, _}, odyssey_frame:decode(Frame))
    after 1000 -> ?assert(false)
    end,
    odyssey_session:acknowledge(Pid, Sequence),
    ?assertEqual(#{pending => 0, sent => 1}, odyssey_session:status(Pid)),
    gen_server:stop(Pid).
