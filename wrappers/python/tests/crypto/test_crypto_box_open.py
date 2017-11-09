import pytest

from indy import IndyError, crypto
from indy.error import ErrorCode

encrypted_message = bytes(
    [147, 139, 214, 56, 147, 118, 153, 21, 25, 160, 156, 58, 195, 146, 143, 205, 84, 189, 227, 99, 145, 182, 183, 66,
     254, 89, 153, 102, 152, 6, 4, 1, 24, 75, 98, 210, 159, 194, 53, 96, 74, 122, 203, 162, 85])
nonce = bytes(
    [131, 1, 41, 49, 232, 199, 187, 95, 87, 96, 234, 102, 33, 251, 22, 230, 30, 97, 234, 85, 94, 244, 93, 89])


@pytest.mark.asyncio
async def test_crypto_box_open_works(wallet_handle, key_my1, verkey_my2, message):
    decrypted_message = await crypto.crypto_box_open(wallet_handle, key_my1, verkey_my2, encrypted_message, nonce)
    assert message == decrypted_message


@pytest.mark.asyncio
async def test_crypto_box_open_works_for_unknown_my_key(wallet_handle, verkey_my1, verkey_my2):
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_open(wallet_handle, verkey_my1, verkey_my2, encrypted_message, nonce)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_crypto_box_open_works_for_other_coder_key(wallet_handle, key_my1, identity_trustee1):
    (_, trustee_verkey) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_open(wallet_handle, key_my1, trustee_verkey, encrypted_message, nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_crypto_box_open_works_for_nonce_not_correspond_message(wallet_handle, key_my1, verkey_my2, message):
    local_nonce = bytes(
        [1, 2, 3, 4, 5, 6, 7, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183])
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_open(wallet_handle, key_my1, verkey_my2, encrypted_message, local_nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_crypto_box_open_works_for_invalid_wallet_handle(wallet_handle, key_my1, verkey_my2, message):
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_open(wallet_handle + 1, key_my1, verkey_my2, encrypted_message, nonce)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
