#!/usr/bin/env escript
%%! +S 2
main(Files) ->
    case os:getenv("ERLFMT_EBIN") of
        false -> io:format(standard_error,"Set ERLFMT_EBIN to erlfmt's compiled modules.~n",[]),halt(1);
        Directory -> code:add_patha(Directory)
    end,
    lists:foreach(fun(File) ->
        case erlfmt:format_file(File,[]) of
            {ok,Text,[]} -> ok=file:write_file(File,Text);
            Error -> io:format(standard_error,"~s: ~p~n",[File,Error]),halt(1)
        end
    end,Files).
