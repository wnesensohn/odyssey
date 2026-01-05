-module(odyssey_session).
-behaviour(gen_server).
-export([start_link/1, send/3, acknowledge/2, status/1]).
-export([init/1, handle_call/3, handle_cast/2, handle_info/2, terminate/2, code_change/3]).

start_link(Options) -> gen_server:start_link(?MODULE, Options, []).
send(Pid, Kind, Payload) -> gen_server:call(Pid, {send, Kind, Payload}).
acknowledge(Pid, Sequence) -> gen_server:cast(Pid, {ack, Sequence}).
status(Pid) -> gen_server:call(Pid, status).

init(Options) ->
    Timer = erlang:send_after(100, self(), retry),
    {ok, #{
        next_sequence => 0,
        window => odyssey_window:new(16, 500),
        sink => maps:get(sink, Options, self()),
        timer => Timer,
        sent => 0
    }}.

handle_call({send, Kind, Payload}, _From, State) ->
    Sequence = maps:get(next_sequence, State),
    case odyssey_frame:encode(Kind, Sequence, Payload) of
        {ok, Frame} ->
            case odyssey_window:reserve(maps:get(window, State), now_ms(), Frame) of
                {ok, Window} ->
                    maps:get(sink, State) ! {downlink, Frame},
                    {reply, {ok, Sequence}, State#{
                        window := Window,
                        next_sequence := (Sequence + 1) band 16#ffff,
                        sent := maps:get(sent, State) + 1
                    }};
                Error ->
                    {reply, Error, State}
            end;
        Error ->
            {reply, Error, State}
    end;
handle_call(status, _From, State) ->
    {reply,
        #{pending => odyssey_window:size(maps:get(window, State)), sent => maps:get(sent, State)},
        State};
handle_call(_, _From, State) ->
    {reply, {error, unsupported}, State}.

handle_cast({ack, Sequence}, State) ->
    {_, Window} = odyssey_window:acknowledge(maps:get(window, State), Sequence),
    {noreply, State#{window := Window}};
handle_cast(_, State) ->
    {noreply, State}.

handle_info(retry, State) ->
    {Frames, Window} = odyssey_window:due(maps:get(window, State), now_ms()),
    lists:foreach(fun(Frame) -> maps:get(sink, State) ! {downlink, Frame} end, Frames),
    Timer = erlang:send_after(100, self(), retry),
    {noreply, State#{window := Window, timer := Timer}};
handle_info(_, State) ->
    {noreply, State}.

terminate(_, State) ->
    erlang:cancel_timer(maps:get(timer, State)),
    ok.
code_change(_, State, _) -> {ok, State}.
now_ms() -> erlang:monotonic_time(millisecond).
