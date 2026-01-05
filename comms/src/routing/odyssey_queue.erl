-module(odyssey_queue).
-export([new/1, push/2, pop/1, size/1]).

new(Capacity) when is_integer(Capacity), Capacity > 0 ->
    #{capacity => Capacity, entries => queue:new(), count => 0}.

push(Queue, Entry) ->
    case maps:get(count, Queue) < maps:get(capacity, Queue) of
        true ->
            {ok, Queue#{
                entries := queue:in(Entry, maps:get(entries, Queue)),
                count := maps:get(count, Queue) + 1
            }};
        false ->
            {error, queue_full}
    end.

pop(Queue) ->
    case queue:out(maps:get(entries, Queue)) of
        {empty, _} ->
            {empty, Queue};
        {{value, Entry}, Rest} ->
            {ok, Entry, Queue#{entries := Rest, count := maps:get(count, Queue) - 1}}
    end.

size(Queue) -> maps:get(count, Queue).
