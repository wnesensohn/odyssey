-module(odyssey_heartbeat).
-export([new/1, observe/2, state/2]).

new(TimeoutMs) when is_integer(TimeoutMs), TimeoutMs > 0 ->
    #{timeout => TimeoutMs, last_seen => undefined, received => 0}.

observe(Heartbeat, NowMs) when is_integer(NowMs) ->
    case maps:get(last_seen, Heartbeat) of
        Last when is_integer(Last), NowMs < Last -> {error, time_reversed};
        _ -> {ok, Heartbeat#{last_seen := NowMs, received := maps:get(received, Heartbeat) + 1}}
    end.

state(Heartbeat, NowMs) ->
    case maps:get(last_seen, Heartbeat) of
        undefined ->
            disconnected;
        Last ->
            Age = NowMs - Last,
            Timeout = maps:get(timeout, Heartbeat),
            if
                Age < 0 -> invalid_clock;
                Age =< Timeout -> connected;
                Age =< Timeout * 2 -> degraded;
                true -> disconnected
            end
    end.
