-module(odyssey_sup).
-behaviour(supervisor).
-export([start_link/0, init/1]).

start_link() -> supervisor:start_link({local, ?MODULE}, ?MODULE, []).
init([]) ->
    Router = #{
        id => odyssey_router,
        start => {odyssey_router, start_link, []},
        restart => permanent,
        shutdown => 5000,
        type => worker,
        modules => [odyssey_router]
    },
    {ok, {#{strategy => one_for_one, intensity => 3, period => 10}, [Router]}}.
