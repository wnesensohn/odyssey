-module(odyssey_window).
-export([new/2, reserve/3, acknowledge/2, due/2, size/1]).

-spec new(pos_integer(), pos_integer()) -> map().
new(Capacity, TimeoutMs) when Capacity > 0, Capacity =< 256, TimeoutMs > 0 ->
    #{capacity => Capacity, timeout_ms => TimeoutMs, pending => #{}}.

-spec reserve(map(), non_neg_integer(), binary()) -> {ok, map()} | {error, atom()}.
reserve(Window, NowMs, Frame) ->
    #{sequence := Sequence} = element(2, odyssey_frame:decode(Frame)),
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
            case maps:get(deadline, Entry) =< NowMs of
                true ->
                    Next = Entry#{
                        deadline := NowMs + Timeout, retries := maps:get(retries, Entry) + 1
                    },
                    {[maps:get(frame, Entry) | Acc], Entries#{Sequence => Next}};
                false ->
                    {Acc, Entries#{Sequence => Entry}}
            end
        end,
        {[], #{}},
        maps:get(pending, Window)
    ),
    {lists:reverse(Frames), Window#{pending := Pending}}.

size(Window) -> map_size(maps:get(pending, Window)).
