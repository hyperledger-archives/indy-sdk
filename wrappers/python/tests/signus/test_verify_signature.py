import asyncio
import json

import pytest

from indy import IndyError
from indy import ledger
from indy import signus
from indy.error import ErrorCode
from ..utils.pool import create_and_open_pool_ledger
from ..utils.wallet import create_and_open_wallet


@pytest.fixture
async def new_did(wallet_handle, pool_handle):
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')

    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                             '{"seed":"00000000000000000000000000000My1"}')

    nym_request = await ledger.build_nym_request(trustee_did, did, ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)
    yield did


@pytest.mark.asyncio
async def test_verify_works_for_verkey_cached_in_wallet(pool_handle, wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                             '{"seed":"000000000000000000000000Trustee1"}')

    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": ver_key}))

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

    valid = await signus.verify_signature(wallet_handle, pool_handle, did, message)
    assert valid


@pytest.mark.asyncio
async def test_verify_works_for_get_verkey_from_ledger(pool_handle, wallet_handle, new_did):
    await signus.store_their_did(wallet_handle, json.dumps({"did": new_did}))

    message = '{"reqId":1496822211362017764,' \
              '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'

    valid = await signus.verify_signature(wallet_handle, pool_handle, new_did, message)
    assert valid


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_runtime_config", ['{"freshness_time":1}'])
async def test_verify_works_for_expired_nym(wallet_handle, pool_handle, wallet_runtime_config):
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')

    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                             '{"seed":"00000000000000000000000000000My1"}')

    nym_request = await ledger.build_nym_request(trustee_did, did, ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    await signus.store_their_did(wallet_handle, json.dumps({"did": did, 'verkey': ver_key}))

    message = '{"reqId":1496822211362017764,' \
              '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'

    await asyncio.sleep(2)

    valid = await signus.verify_signature(wallet_handle, pool_handle, did, message)
    assert valid


@pytest.mark.asyncio
async def test_verify_works_for_invalid_wallet(pool_handle, wallet_handle, new_did):
    with pytest.raises(IndyError) as e:
        message = '{"reqId":1496822211362017764,' \
                  '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'
        await signus.verify_signature(wallet_handle + 1, pool_handle, new_did, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_invalid_pool(pool_handle, wallet_handle, new_did):
    with pytest.raises(IndyError) as e:
        message = '{"reqId":1496822211362017764,' \
                  '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'
        await signus.verify_signature(wallet_handle, pool_handle + 1, new_did, message)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_other_signer(pool_handle, wallet_handle, new_did):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                             '{"seed":"000000000000000000000000Steward1"}')

    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": ver_key}))

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

    valid = await signus.verify_signature(wallet_handle, pool_handle, did, message)
    assert not valid


@pytest.mark.asyncio
async def test_verify_works_for_invalid_message_format(pool_handle, wallet_handle, new_did):
    with pytest.raises(IndyError) as e:
        message = '"reqId":1496822211362017764,' \
                  '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"'
        await signus.verify_signature(wallet_handle, pool_handle, new_did, message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_message_without_signature(pool_handle, wallet_handle, new_did):
    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle, new_did, '{"reqId":1496822211362017764}')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_get_nym_from_ledger_with_incompatible_wallet(cleanup_storage):
    with pytest.raises(IndyError) as e:
        pool_handle = await create_and_open_pool_ledger("pool_name")
        wallet_handle = await create_and_open_wallet(pool_name="other_pool_name", wallet_name="incompatible_wallet")

        (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                                 '{"seed":"00000000000000000000000000000My1"}')

        await signus.store_their_did(wallet_handle, json.dumps({"did": did}))

        message = '{"reqId":1496822211362017764,' \
                  '"signature":"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai"}'

        await signus.verify_signature(wallet_handle, pool_handle, did, message)

    assert ErrorCode.WalletIncompatiblePoolError == e.value.error_code
