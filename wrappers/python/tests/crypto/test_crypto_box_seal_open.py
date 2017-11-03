import pytest

from indy import IndyError, crypto
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_crypto_box_seal_open_works(wallet_handle, key_my1, message):
    encrypted_message = await crypto.crypto_box_seal(key_my1, message)
    decrypted_message = await crypto.crypto_box_seal_open(wallet_handle, key_my1, encrypted_message)
    assert message == decrypted_message


@pytest.mark.asyncio
async def test_crypto_box_seal_open_works_for_other_key(wallet_handle, key_my1, verkey_my2, message):
    encrypted_message = await crypto.crypto_box_seal(verkey_my2, message)
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_seal_open(wallet_handle, key_my1, encrypted_message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_crypto_box_seal_open_works_for_unknown_key(wallet_handle, verkey_my1, message):
    encrypted_message = await crypto.crypto_box_seal(verkey_my1, message)
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_seal_open(wallet_handle, verkey_my1, encrypted_message)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_crypto_box_seal_open_works_for_invalid_wallet_handle(wallet_handle, key_my1, message):
    encrypted_message = await crypto.crypto_box_seal(key_my1, message)
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_seal_open(wallet_handle + 1, key_my1, encrypted_message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
