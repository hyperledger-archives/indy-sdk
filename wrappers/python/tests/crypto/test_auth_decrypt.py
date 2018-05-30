import json

from indy import IndyError
from indy import crypto

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_auth_decrypt_works(wallet_handle, identity_steward1, identity_trustee1, message):
    (_, my_verkey) = identity_steward1
    (_, their_verkey) = identity_trustee1
    encrypted_msg = await crypto.auth_crypt(wallet_handle, my_verkey, their_verkey, message)
    verkey, parsed_message = await crypto.auth_decrypt(wallet_handle, their_verkey, encrypted_msg)
    assert my_verkey == verkey
    assert message == parsed_message


@pytest.mark.asyncio
async def test_auth_decrypt_works_for_invalid_msg(wallet_handle, identity_steward1, verkey_my1):
    (_, my_verkey) = identity_steward1

    msg = json.dumps({
        'auth': True,
        'nonce': [1, 2, 3, 4, 5, 6],
        'sender': verkey_my1,
        'msg': [16, 85, 246, 243, 120, 246, 219, 123, 127, 175, 76, 243, 223, 143, 20, 163, 77, 88, 56, 211, 173,
                108, 252, 30, 210, 202, 183, 215, 102, 93, 101, 185, 51, 114, 89, 24, 207, 123, 156, 228, 6, 39, 55,
                250, 172]
    })
    with pytest.raises(IndyError) as e:
        await crypto.auth_decrypt(wallet_handle, my_verkey, msg.encode('utf-8'))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_auth_decrypt_works_for_unknown_recipient_vk(wallet_handle, identity_steward1, verkey_my1, message):
    (_, my_verkey) = identity_steward1
    encrypted_msg = await crypto.auth_crypt(wallet_handle, my_verkey, verkey_my1, message)
    with pytest.raises(IndyError) as e:
        await crypto.auth_decrypt(wallet_handle, verkey_my1, encrypted_msg)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_auth_decrypt_works_for_invalid_handle(wallet_handle, identity_steward1, identity_trustee1, message):
    (_, my_verkey) = identity_steward1
    (_, their_verkey) = identity_trustee1
    encrypted_msg = await crypto.auth_crypt(wallet_handle, my_verkey, their_verkey, message)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await crypto.auth_decrypt(invalid_wallet_handle, their_verkey, encrypted_msg)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
