import json

import pytest

from indy import IndyError, signus
from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'.encode('utf-8')
encrypted_message = bytes(
    [16, 85, 246, 243, 120, 246, 219, 123, 127, 175, 76, 243, 223, 143, 20, 163, 77, 88, 56, 211, 173, 108, 252, 30,
     210, 202, 183, 215, 102, 93, 101, 185, 51, 114, 89, 24, 207, 123, 156, 228, 6, 39, 55, 250, 172])
nonce = bytes(
    [201, 29, 226, 18, 20, 147, 99, 221, 4, 148, 15, 193, 189, 126, 219, 236, 83, 199, 210, 242, 245, 173, 119, 74])


@pytest.mark.asyncio
async def test_decrypt_works(wallet_handle, identity_my2, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my2

    decrypted_message = await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_message, nonce)
    assert message == decrypted_message


@pytest.mark.asyncio
async def test_decrypt_works_for_other_coder(pool_handle, wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee_verkey) = identity_trustee1
    (my_did, my_verkey) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee_verkey}))
    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, "verkey": my_verkey}))

    (encrypted_msg, local_nonce) = await signus.encrypt(wallet_handle, pool_handle, my_did, my_did, message)

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_msg, local_nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_decrypt_works_for_nonce_not_correspond_message(wallet_handle, identity_my2, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my2

    local_nonce = bytes(
        [1, 2, 3, 4, 5, 6, 7, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183])

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_message, local_nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_decrypt_works_for_invalid_wallet_handle(wallet_handle, identity_my2, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my2

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle + 1, my_did, trustee_did, encrypted_message, nonce)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
