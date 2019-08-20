import pytest

from indy import crypto, error


@pytest.mark.asyncio
async def test_crypto_sign_works(wallet_handle, key_my1, message):
    expected_signature = bytes(
        [169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120,
         94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191,
         41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11])

    signature = await crypto.crypto_sign(wallet_handle, key_my1, message)
    assert signature == expected_signature


@pytest.mark.asyncio
async def test_crypto_sign_works_for_unknown_signer(wallet_handle, message, verkey_my1):
    with pytest.raises(error.WalletItemNotFound):
        await crypto.crypto_sign(wallet_handle, verkey_my1, message)
