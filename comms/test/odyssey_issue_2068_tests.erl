-module(odyssey_issue_2068_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assertEqual(next, odyssey_frame:sequence_class(65535, 0, 16)).

boundary_test() -> ?assertEqual(duplicate, odyssey_frame:sequence_class(10, 10, 16)).
