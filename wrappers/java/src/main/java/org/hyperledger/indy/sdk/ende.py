from enum import IntEnum
import json
from math import ceil, log
from typing import Any, Union
from random import choice
from string import printable
import sys
from decimal import *


def raw(orig: Any) -> dict:
    """
    Stringify input value, empty string for None.
    :param orig: original attribute value of any stringifiable type
    :return: stringified raw value
    """

    return '' if orig is None else str(orig)


class Prefix(IntEnum):
    """
    Prefixes for indy encoding to numeric strings. For indy-sdk, 32-bit integers must encode
    to themselves to allow predicates to work.
    A single-digit prefix to identify original type allows the decode to return it, without
    taking the encoding outside the space of numeric strings.
    """

    I32 = 0  # purely a formalism, no prefix for indy (32-bit) int values
    STR = 1
    BOOL = 2
    POSINT = 3
    NEGINT = 4
    FLOAT = 5
    JSON = 9


I32_BOUND = 2**31


def _prefix(orig: Any) -> Prefix:
    """
    Return the prefix for an original value to encode.
    :param orig: input value to encode
    :return: Prefix enum value
    """

    if isinstance(orig, str):
        return Prefix.JSON if orig and all(orig[i] == chr(0) for i in range(len(orig))) else Prefix.STR
    if isinstance(orig, bool):
        return Prefix.BOOL
    if isinstance(orig, int):
        if -I32_BOUND <= orig < I32_BOUND:
            return Prefix.I32
        return Prefix.POSINT if orig >= I32_BOUND else Prefix.NEGINT
    if isinstance(orig, Decimal):
        return Prefix.FLOAT
    return Prefix.JSON


def encode(orig: Any) -> str:
    """
    Encode credential attribute value, leaving any (stringified) int32 alone: indy-sdk predicates
    operate on int32 values properly only when their encoded values match their raw values.
    To disambiguate for decoding, the operation reserves a sentinel for special values and otherwise adds
    2**31 to any non-trivial transform of a non-int32 input, then prepends a digit marking the input type:
      * 1: string (except non-empty string with all characters chr(0))
      * 2: boolean
      * 3: positive non-32-bit integer
      * 4: negative non-32-bit integer
      * 5: floating point
      * 9: other (JSON-encodable) - including non-empty string with all characters chr(0).
    The original value must be JSON-encodable.
    :param orig: original JSON-encodable value to encode
    :return: encoded value
    """

    if orig is None:
        return str(I32_BOUND)  # sentinel

    prefix = '{}'.format(_prefix(orig) or '')  # no prefix for indy 32-bit ints

    if isinstance(orig, bool):
        return '{}{}'.format(
            prefix,
            I32_BOUND + 2 if orig else I32_BOUND + 1)  # python bool('False') = True; just use 2 sentinels

    if isinstance(orig, int):
        return '{}{}'.format(prefix, str(orig) if -I32_BOUND <= orig < I32_BOUND else str(abs(orig)))

    if isinstance(orig, Decimal):
        return '{}{}'.format(
        prefix,
        str(int.from_bytes(bytes(str(orig), encoding = 'utf-8'), 'big') + I32_BOUND))

    rv = '{}{}'.format(
        prefix,
        str(int.from_bytes(
            orig.encode() if int(prefix) == Prefix.STR else json.dumps(orig).encode(), 'big') + I32_BOUND))

    return rv


def decode(enc_value: str) -> Union[str, None, bool, int, float]:
    """
    Decode encoded credential attribute value.
    :param enc_value: numeric string to decode
    :return: decoded value, stringified if original was neither str, bool, int, nor float
    """

    assert enc_value.isdigit() or enc_value[0] == '-' and enc_value[1:].isdigit()

    if -I32_BOUND <= int(enc_value) < I32_BOUND:  # it's an i32: it is its own encoding
        return int(enc_value)
    if int(enc_value) == I32_BOUND:
        return None  # sentinel

    (prefix, payload) = (int(enc_value[0]), int(enc_value[1:]))
    ival = int(payload) - I32_BOUND

    if prefix == Prefix.STR and ival == 0:
        return ''  # special case: empty string encodes as 2**31
    if prefix == Prefix.BOOL and ival in (1, 2):
        return False if ival == 1 else True  # sentinels
    if prefix in (Prefix.POSINT, Prefix.NEGINT):
        return int(payload) if prefix == Prefix.POSINT else -int(payload)

    blen = max(ceil(log(ival, 16)/2), 1)
    ibytes = ival.to_bytes(blen, 'big')

    if prefix == Prefix.FLOAT:
        return Decimal(ibytes.decode())

    return ibytes.decode() if prefix == Prefix.STR else json.loads(ibytes.decode())


def cred_attr_value(orig: Any) -> dict:
    """
    Given a value, return corresponding credential attribute value dict for indy-sdk processing.
    :param orig: original attribute value of any stringifiable type
    :return: dict on 'raw' and 'encoded' keys for indy-sdk processing
    """
    return {'raw': raw(orig), 'encoded': encode(orig)}


def test_enco_deco():
    print('\n\n== Starting encode/decode for string of length up to 1024')

    for printable_len in range(0, 1025):
        orig = ''.join(choice(printable) for _ in range(printable_len))
        print('.', end='' if (printable_len + 1) % 100 else '{}\n'.format(printable_len), flush=True)
        enc = encode(orig)
        dec = decode(enc)
        assert cred_attr_value(orig) == {'raw': raw(orig), 'encoded': enc}
        assert orig == dec
    print('\n\n== Random printable string test passed')

    print('\n\n== Typical cases - (type) orig -> encoded -> (type) decoded:')
    for orig in (
            chr(0),
            chr(1),
            chr(2),
            'Alice',
            'Bob',
            'J.R. "Bob" Dobbs',
            None,
            True,
            False,
            -5,
            0,
            1024,
            2**31 - 1,
            2**31,
            2**31 + 1,
            -2**31 - 1,
            -2**31,
            -2**31 + 1,
            Decimal("0.0"),
            '0.0',
            Decimal("0.1"),
            Decimal("-0.1"),
            Decimal(-1.9234856120348166e+37),
            Decimal("1.9234856120348166e+37"),
            Decimal("-19234856120348165921835629183561023142.55"),
            Decimal("19234856120348165921835629183561023142.55"),
            Decimal.from_float(sys.float_info.max),
            'Hello',
            '',
            'True',
            'False',
            '1234',
            '-12345',
            [],
            [0, 1, 2, 3],
            {'a': 1, 'b': 2, 'c': 3},
            [{}, {'a': [0, 0.1], 'b': [0.0, 19234856120348165921835629183561023142.55]}, True],
            ):
        enc = encode(orig)
        dec = decode(enc)
        print('  ({})({}) -> {} -> ({})({})'.format(
            type(orig).__name__,
            '0x{:02x}'.format(ord(orig))
                if orig in (chr(0), chr(1), chr(2))
                else "%f" % orig if isinstance(orig, float)
                else orig,
            enc,
            type(dec).__name__,
            '0x{:02x}'.format(ord(dec))
                if dec in (chr(0), chr(1), chr(2))
                else "%f" % dec if isinstance(dec, float)
                else dec))
        assert orig == dec

    for i in range(32):
        orig = ''.join(map(chr, [0] * i))
        enc = encode(orig)
        dec = decode(enc)
        assert cred_attr_value(orig) == {'raw': raw(orig), 'encoded': enc}
        assert orig == dec
    print('Tests OK for (str)(chr(0) multiples)')
    print("{0:.10f}".format(-1.9234856120348166e+37))  # -19234856120348165827208446428657483776.0000000000
    print(Decimal.from_float(-1.9234856120348166e+37))


if __name__ == "__main__":
    test_enco_deco()