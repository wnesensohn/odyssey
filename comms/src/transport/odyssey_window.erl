-module(odyssey_window).
-export([ack_latency/2]).
-export([retry_budget/2]).
-export([new/2, reserve/3, acknowledge/2, due/2, size/1]).

-spec new(pos_integer(), pos_integer()) -> map().
new(Capacity, TimeoutMs) when Capacity > 0, Capacity =< 256, TimeoutMs > 0 ->
    #{capacity => Capacity, timeout_ms => TimeoutMs, pending => #{}}.

-spec reserve(map(), non_neg_integer(), binary()) -> {ok, map()} | {error, atom()}.
reserve(Window, NowMs, Frame) ->
    case odyssey_frame:decode(Frame) of
        {ok, #{sequence := Sequence}} -> reserve_decoded(Window, NowMs, Sequence, Frame);
        {error, Reason} -> {error, Reason}
    end.

reserve_decoded(Window, NowMs, Sequence, Frame) ->
    Pending = maps:get(pending, Window),
    case maps:is_key(Sequence, Pending) orelse map_size(Pending) >= maps:get(capacity, Window) of
        true ->
            {error, window_full};
        false ->
            Entry = #{
                frame => Frame, deadline => NowMs + maps:get(timeout_ms, Window), retries => 0
            },
            {ok, Window#{pending := Pending#{Sequence => Entry}}}
    end.

-spec acknowledge(map(), non_neg_integer()) -> {boolean(), map()}.
acknowledge(Window, Sequence) ->
    Pending = maps:get(pending, Window),
    {maps:is_key(Sequence, Pending), Window#{pending := maps:remove(Sequence, Pending)}}.

-spec due(map(), non_neg_integer()) -> {[binary()], map()}.
due(Window, NowMs) ->
    Timeout = maps:get(timeout_ms, Window),
    {Frames, Pending} = maps:fold(
        fun(Sequence, Entry, {Acc, Entries}) ->
            case {maps:get(deadline, Entry) =< NowMs, retry_budget(maps:get(retries, Entry), 3)} of
                {true, retry} ->
                    Next = Entry#{
                        deadline := NowMs + Timeout, retries := maps:get(retries, Entry) + 1
                    },
                    {[maps:get(frame, Entry) | Acc], Entries#{Sequence => Next}};
                {true, exhausted} ->
                    {Acc, Entries};
                {false, _} ->
                    {Acc, Entries#{Sequence => Entry}}
            end
        end,
        {[], #{}},
        maps:get(pending, Window)
    ),
    {lists:reverse(Frames), Window#{pending := Pending}}.

size(Window) -> map_size(maps:get(pending, Window)).

retry_budget(Retries, Maximum) when
    is_integer(Retries), Retries >= 0, is_integer(Maximum), Maximum >= 0
->
    case Retries + 1 < Maximum of
        true -> retry;
        false -> exhausted
    end;
retry_budget(_, _) ->
    {error, invalid_retry_budget}.

ack_latency(SentMs, AckMs) when is_integer(SentMs), is_integer(AckMs), AckMs >= SentMs ->
    {ok, AckMs - SentMs};
ack_latency(_, _) ->
    {error, time_reversed}.
