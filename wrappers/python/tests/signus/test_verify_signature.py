import asyncio

import json

import pytest

from indy_sdk import IndyError, ledger, signus, wallet
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_verify_works_for_verkey_cached_in_wallet(pool_handle, wallet_handle, identity_trustee1):
    (trustee_did, trustee__verkey) = identity_trustee1

    await signus.store_their_did(wallet_handle, json.dumps({"did": trustee_did, "verkey": trustee__verkey}))

    message = json.dumps({
        "reqId": 1496822211362017764,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "VsKV7grR1BUE29mG2Fm2kX",
            "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
        },
        "signature": "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"
    })

    valid = await signus.verify_signature(wallet_handle, pool_handle, trustee_did, message)
    assert valid


@pytest.mark.asyncio
async def test_verify_works_for_get_verkey_from_ledger(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did}))

    message = '{"reqId":1496822211362017764,' \
              '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'

    valid = await signus.verify_signature(wallet_handle, pool_handle, my_did, message)
    assert valid


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_runtime_config", ['{"freshness_time":1}'])
async def test_verify_works_for_expired_nym(wallet_handle, pool_handle, identity_trustee1, identity_my1, wallet_runtime_config):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, 'verkey': my_verkey}))

    message = '{"reqId":1496822211362017764,' \
              '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'

    await asyncio.sleep(2)

    valid = await signus.verify_signature(wallet_handle, pool_handle, my_did, message)
    assert valid


@pytest.mark.asyncio
async def test_verify_works_for_invalid_wallet(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1

    with pytest.raises(IndyError) as e:
        message = '{"reqId":1496822211362017764,' \
                  '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'
        await signus.verify_signature(wallet_handle + 1, pool_handle, my_did, message)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_invalid_pool(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1

    with pytest.raises(IndyError) as e:
        message = '{"reqId":1496822211362017764,' \
                  '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'
        await signus.verify_signature(wallet_handle, pool_handle + 1, my_did, message)

    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_other_signer(pool_handle, wallet_handle, identity_steward1, identity_my1):
    (steward_did, steward_verkey) = identity_steward1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": steward_did, "verkey": steward_verkey}))

    message = json.dumps({
        "reqId": 1496822211362017764,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "VsKV7grR1BUE29mG2Fm2kX",
            "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
        },
        "signature": "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"
    })

    valid = await signus.verify_signature(wallet_handle, pool_handle, steward_did, message)
    assert not valid


@pytest.mark.asyncio
async def test_verify_works_for_invalid_message_format(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1

    message = '"reqId":1496822211362017764,' \
              '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"'

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle, my_did, message)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_message_without_signature(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle, my_did, '{"reqId":1496822211362017764}')

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_get_nym_from_ledger_with_incompatible_wallet(pool_name, wallet_name, pool_handle,
                                                                             seed_my1):
    pool_name = "other_" + pool_name
    wallet_name = "other_" + wallet_name

    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({"seed": seed_my1}))

    await signus.store_their_did(wallet_handle, json.dumps({"did": did}))

    message = '{"reqId":1496822211362017764,' \
              '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle, did, message)

    assert ErrorCode.WalletIncompatiblePoolError == e.value.error_code
