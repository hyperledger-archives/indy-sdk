import json

import pytest

from indy import IndyError, signus
from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'.encode('utf-8')
encrypted_message = bytes([187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5,
                     216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75,
                     73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32])
nonce = bytes([242, 246, 53, 153, 106, 37, 185, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183])


@pytest.mark.asyncio
async def test_decrypt_works(wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))

    decrypted_message = await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_message, nonce)
    assert message == decrypted_message


@pytest.mark.asyncio
async def test_decrypt_works_for_other_coder(pool_handle, wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, my_verkey) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))
    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, "verkey": my_verkey}))

    (encrypted_msg, local_nonce) = await signus.encrypt(wallet_handle, pool_handle, my_did, my_did, message)

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_msg, local_nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_decrypt_works_for_nonce_not_correspond_message(wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))
    local_nonce = bytes(
        [1, 2, 3, 4, 5, 6, 7, 65, 212, 14, 109, 131, 200, 169, 94, 110, 51, 47, 101, 89, 0, 171, 105, 183])

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_message, local_nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_decrypt_works_for_invalid_wallet_handle(wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle + 1, my_did, trustee_did, encrypted_message, nonce)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
