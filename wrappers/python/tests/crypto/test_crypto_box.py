import pytest

from indy import IndyError, crypto
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_crypto_box_works(wallet_handle, key_my1, verkey_my2, message):
    await crypto.crypto_box(wallet_handle, key_my1, verkey_my2, message)


@pytest.mark.asyncio
async def test_crypto_box_works_for_unknown_coder(wallet_handle, message, verkey_my1, verkey_my2):
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box(wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_crypto_box_works_for_invalid_wallet_handle(wallet_handle, key_my1, verkey_my2, message):
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box(wallet_handle + 1, key_my1, verkey_my2, message)
        assert ErrorCode.WalletInvalidHandle == e.value.error_code
