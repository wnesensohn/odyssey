-module(odyssey_issue_2032_tests).
-include_lib("eunit/include/eunit.hrl").

nominal_test() -> ?assert(odyssey_frame:valid_kind(1)).

boundary_test() -> ?assert(odyssey_frame:valid_kind(3)).
