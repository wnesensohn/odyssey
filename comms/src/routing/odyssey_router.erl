-module(odyssey_router).
-behaviour(gen_server).
-export([start_link/0, subscribe/2, publish/3]).
-export([init/1, handle_call/3, handle_cast/2, handle_info/2, terminate/2, code_change/3]).

start_link() -> gen_server:start_link({local, ?MODULE}, ?MODULE, [], []).
subscribe(Topic, Pid) -> gen_server:call(?MODULE, {subscribe, Topic, Pid}).
publish(Topic, Sequence, Payload) -> gen_server:cast(?MODULE, {publish, Topic, Sequence, Payload}).
init([]) -> {ok, #{subscribers => #{}, monitors => #{}}}.

handle_call({subscribe, Topic, Pid}, _From, State) when is_atom(Topic), is_pid(Pid) ->
    Subscribers = maps:get(subscribers, State),
    Existing = maps:get(Topic, Subscribers, []),
    case lists:member(Pid, Existing) of
        true ->
            {reply, ok, State};
        false ->
            Ref = erlang:monitor(process, Pid),
            Monitors = maps:get(monitors, State),
            {reply, ok, State#{
                subscribers := Subscribers#{Topic => [Pid | Existing]},
                monitors := Monitors#{Ref => {Topic, Pid}}
            }}
    end;
handle_call(_, _From, State) ->
    {reply, {error, unsupported}, State}.

handle_cast({publish, Topic, Sequence, Payload}, State) ->
    lists:foreach(
        fun(Pid) -> Pid ! {telemetry, Topic, Sequence, Payload} end,
        maps:get(Topic, maps:get(subscribers, State), [])
    ),
    {noreply, State};
handle_cast(_, State) ->
    {noreply, State}.

handle_info({'DOWN', Ref, process, _Pid, _Reason}, State) ->
    Monitors = maps:get(monitors, State),
    case maps:take(Ref, Monitors) of
        {{Topic, Pid}, Remaining} ->
            Subscribers = maps:get(subscribers, State),
            {noreply, State#{
                monitors := Remaining,
                subscribers := Subscribers#{
                    Topic => lists:delete(Pid, maps:get(Topic, Subscribers))
                }
            }};
        error ->
            {noreply, State}
    end;
handle_info(_, State) ->
    {noreply, State}.
terminate(_, _) -> ok.
code_change(_, State, _) -> {ok, State}.
