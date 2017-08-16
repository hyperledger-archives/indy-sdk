import json

import pytest

from indy import IndyError, signus
from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'
encrypted_message = "4SWFzd3sx7xNemZEtktt3s558Fa28fGbauAZv9NRQjQhHq9bwT8uBnACQJAKzZ"


@pytest.mark.asyncio
async def test_decrypt_works(wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))

    nonce = "Dd3vSQgDdADJGoxb6BPcWU6wkEMqSeFwv"
    decrypted_message = await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_message, nonce)
    assert message == decrypted_message


@pytest.mark.asyncio
async def test_decrypt_works_for_other_coder(pool_handle, wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, my_verkey) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))
    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, "verkey": my_verkey}))

    (encrypted_msg, nonce) = await signus.encrypt(wallet_handle, pool_handle, my_did, my_did, message)

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_msg, nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_decrypt_works_for_nonce_not_correspond_message(wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))
    nonce = "acS2SQgDdfE3Goxa1AhcWCa4kEMqSelv7"

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle, my_did, trustee_did, encrypted_message, nonce)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_decrypt_works_for_invalid_wallet_handle(wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))
    nonce = "Dd3vSQgDdADJGoxb6BPcWU6wkEMqSeFwv"

    with pytest.raises(IndyError) as e:
        await signus.decrypt(wallet_handle + 1, my_did, trustee_did, encrypted_message, nonce)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
