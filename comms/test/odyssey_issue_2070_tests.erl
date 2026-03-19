-module(odyssey_issue_2070_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual(accept, odyssey_gateway:admit_worker(3, 4)).
