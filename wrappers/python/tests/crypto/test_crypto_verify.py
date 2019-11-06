import pytest

from indy import crypto, error

signature = bytes(
    [169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94,
     204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48,
     30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11])


@pytest.mark.asyncio
async def test_crypto_verify_works(verkey_my1, message):
    valid = await crypto.crypto_verify(verkey_my1, message, signature)
    assert valid


@pytest.mark.asyncio
async def test_crypto_verify_works_for_other_signer(verkey_my2, message):
    valid = await crypto.crypto_verify(verkey_my2, message, signature)
    assert not valid


@pytest.mark.asyncio
async def test_crypto_verify_works_for_verkey_with_incorrect_crypto_type(verkey_my1, message):
    verkey = verkey_my1 + ':unknown_crypto'
    with pytest.raises(error.UnknownCryptoTypeError):
        await crypto.crypto_verify(verkey, message, signature)
