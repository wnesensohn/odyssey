-module(odyssey_gateway).
-export([start/2, stop/1]).

-spec start(inet:port_number(), fun((map()) -> binary())) ->
    {ok, pid(), inet:port_number()} | {error, term()}.
start(Port, Handler) when is_function(Handler, 1) ->
    case gen_tcp:listen(Port, [binary, {active, false}, {reuseaddr, true}, {ip, {127, 0, 0, 1}}]) of
        {ok, Listen} ->
            {ok, {_, BoundPort}} = inet:sockname(Listen),
            Pid = spawn_link(fun() -> accept_loop(Listen, Handler) end),
            {ok, Pid, BoundPort};
        Error ->
            Error
    end.

stop(Pid) ->
    unlink(Pid),
    exit(Pid, shutdown),
    ok.

accept_loop(Listen, Handler) ->
    case gen_tcp:accept(Listen) of
        {ok, Socket} ->
            Worker = spawn(fun() ->
                receive
                    {socket, Owned} -> serve(Owned, Handler, <<>>)
                end
            end),
            ok = gen_tcp:controlling_process(Socket, Worker),
            Worker ! {socket, Socket},
            accept_loop(Listen, Handler);
        {error, closed} ->
            ok;
        {error, _} ->
            gen_tcp:close(Listen),
            ok
    end.

serve(Socket, Handler, Buffer) ->
    case gen_tcp:recv(Socket, 0, 5000) of
        {ok, Data} when byte_size(Buffer) + byte_size(Data) =< 65536 ->
            case odyssey_frame:split_stream(<<Buffer/binary, Data/binary>>) of
                {ok, Frames, Tail} ->
                    case process_frames(Socket, Handler, Frames) of
                        ok -> serve(Socket, Handler, Tail);
                        error -> gen_tcp:close(Socket)
                    end;
                {error, _} ->
                    gen_tcp:close(Socket)
            end;
        _ ->
            gen_tcp:close(Socket)
    end.

process_frames(_, _, []) ->
    ok;
process_frames(Socket, Handler, [Frame | Tail]) ->
    case odyssey_frame:decode(Frame) of
        {ok, #{sequence := Sequence} = Decoded} ->
            try Handler(Decoded) of
                Payload when is_binary(Payload) ->
                    case odyssey_frame:encode(2, Sequence, Payload) of
                        {ok, Response} ->
                            case gen_tcp:send(Socket, Response) of
                                ok -> process_frames(Socket, Handler, Tail);
                                _ -> error
                            end;
                        _ ->
                            error
                    end
            catch
                _:_ -> error
            end;
        _ ->
            error
    end.
