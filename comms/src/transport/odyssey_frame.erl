-module(odyssey_frame).
-export([capabilities/1]).
-export([inspect/1]).
-export([sequence_class/3]).
-export([valid_kind/1]).
-export([encode/3, decode/1, crc16/1, split_stream/1]).

-define(MAX_PAYLOAD, 1024).

-spec encode(0..255, 0..4294967295, binary()) -> {ok, binary()} | {error, atom()}.
encode(Kind, Sequence, Payload) when
    is_integer(Kind),
    Kind >= 1,
    Kind =< 3,
    is_integer(Sequence),
    Sequence >= 0,
    Sequence =< 4294967295,
    is_binary(Payload),
    byte_size(Payload) =< ?MAX_PAYLOAD
->
    Header = <<"OD", 4, Kind, Sequence:32/big, (byte_size(Payload)):16/big, Payload/binary>>,
    {ok, <<Header/binary, (crc16(Header)):16/big>>};
encode(_, _, _) ->
    {error, invalid_frame}.

-spec decode(binary()) -> {ok, map()} | {error, atom()}.
decode(<<"OD", 4, Kind, Sequence:32/big, Length:16/big, Rest/binary>> = Frame) when
    Kind >= 1, Kind =< 3, Length =< ?MAX_PAYLOAD, byte_size(Rest) =:= Length + 2
->
    <<Payload:Length/binary, Expected:16/big>> = Rest,
    Content = binary:part(Frame, 0, byte_size(Frame) - 2),
    case crc16(Content) of
        Expected -> {ok, #{kind => Kind, sequence => Sequence, payload => Payload}};
        _ -> {error, checksum}
    end;
decode(_) ->
    {error, invalid_frame}.

-spec crc16(binary()) -> 0..65535.
crc16(Bytes) ->
    lists:foldl(
        fun(Byte, Crc) -> crc_bits(Crc bxor (Byte bsl 8), 8) end,
        16#ffff,
        binary_to_list(Bytes)
    ).

crc_bits(Crc, 0) ->
    Crc band 16#ffff;
crc_bits(Crc, Count) ->
    Next =
        case Crc band 16#8000 of
            0 -> Crc bsl 1;
            _ -> (Crc bsl 1) bxor 16#1021
        end,
    crc_bits(Next band 16#ffff, Count - 1).

-spec split_stream(binary()) -> {ok, [binary()], binary()} | {error, atom()}.
split_stream(Bytes) -> split_stream(Bytes, []).

split_stream(Bytes, Acc) when byte_size(Bytes) < 10 ->
    {ok, lists:reverse(Acc), Bytes};
split_stream(<<"OD", 4, _Kind, _Sequence:32, Length:16, _/binary>> = Bytes, Acc) when
    Length =< ?MAX_PAYLOAD
->
    Size = Length + 12,
    case byte_size(Bytes) >= Size of
        true ->
            <<Frame:Size/binary, Tail/binary>> = Bytes,
            split_stream(Tail, [Frame | Acc]);
        false ->
            {ok, lists:reverse(Acc), Bytes}
    end;
split_stream(_, _) ->
    {error, invalid_stream}.

valid_kind(Kind) -> Kind =:= 1 orelse Kind =:= 2 orelse Kind =:= 3.

sequence_class(Previous, Current, Window) when
    is_integer(Previous), is_integer(Current), is_integer(Window), Window > 0, Window < 2147483648
->
    Delta = (Current - Previous) band 16#ffffffff,
    if
        Delta =:= 0 -> duplicate;
        Delta =< Window -> next;
        true -> stale
    end;
sequence_class(_, _, _) ->
    invalid.

inspect(Bytes) ->
    case decode(Bytes) of
        {ok, Frame} ->
            {ok, (maps:without([payload], Frame))#{
                payload_bytes => byte_size(maps:get(payload, Frame))
            }};
        Error ->
            Error
    end.

capabilities(Version) when Version =:= 3; Version =:= 4 ->
    #{wire_version => Version, max_payload => 1024, checksum => ccitt_false};
capabilities(_) ->
    {error, unsupported_version}.
