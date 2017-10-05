import json

import pytest

from indy import IndyError, signus
from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'.encode('utf-8')


@pytest.mark.asyncio
async def test_anonymous_decrypt_works(pool_handle, wallet_handle, identity_trustee1):
    (did, verkey) = identity_trustee1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": verkey}))

    encrypted_message = await signus.anonymous_encrypt(wallet_handle, pool_handle, did, message)
    decrypted_message = await signus.anonymous_decrypt(wallet_handle, did, encrypted_message)
    assert message == decrypted_message


@pytest.mark.asyncio
async def test_anonymous_decrypt_works_for_other_coder(pool_handle, wallet_handle, identity_steward1,
                                                       identity_trustee1):
    (trustee_did, trustee_verkey) = identity_trustee1
    (steward_did, steward_verkey) = identity_steward1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee_verkey}))
    await signus.store_their_did(wallet_handle, json.dumps({"did": steward_did, "verkey": steward_verkey}))

    encrypted_msg = await signus.anonymous_encrypt(wallet_handle, pool_handle, steward_did, message)
    with pytest.raises(IndyError) as e:
        await signus.anonymous_decrypt(wallet_handle, trustee_did, encrypted_msg)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_anonymous_decrypt_works_for_invalid_wallet_handle(wallet_handle, identity_trustee1):
    (did, verkey) = identity_trustee1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": verkey}))

    with pytest.raises(IndyError) as e:
        encrypted_message = bytes([187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228])
        await signus.anonymous_decrypt(wallet_handle + 1, did, encrypted_message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
