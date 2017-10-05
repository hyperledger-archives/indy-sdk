import asyncio

import json

import pytest

from indy import IndyError, signus
from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'.encode('utf-8')


@pytest.mark.asyncio
async def test_encrypt_works_for_pk_cached_in_wallet(pool_handle, wallet_handle, identity_trustee1, identity_my2):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my2

    await signus.encrypt(wallet_handle, pool_handle, my_did, trustee_did, message)


@pytest.mark.asyncio
async def test_encrypt_works_for_get_pk_from_ledger(pool_handle, wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did}))
    await signus.encrypt(wallet_handle, pool_handle, trustee_did, my_did, message)


@pytest.mark.asyncio
async def test_encrypt_works_for_get_nym_from_ledger(pool_handle, wallet_handle, identity_my1, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.encrypt(wallet_handle, pool_handle, trustee_did, my_did, message)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_runtime_config", ['{"freshness_time":1}'])
async def test_encrypt_works_for_expired_nym(wallet_handle, pool_handle, identity_trustee1, identity_my1,
                                             wallet_runtime_config):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, 'verkey': my_verkey}))

    await asyncio.sleep(2)

    await signus.encrypt(wallet_handle, pool_handle, trustee_did, my_did, message)


@pytest.mark.asyncio
async def test_encrypt_works_for_invalid_wallet_handle(wallet_handle, pool_handle, identity_my2, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my2

    with pytest.raises(IndyError) as e:
        await signus.encrypt(wallet_handle + 1, pool_handle, my_did, trustee_did, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_encrypt_works_for_invalid_pool_handle(wallet_handle, pool_handle, identity_my2, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my2

    with pytest.raises(IndyError) as e:
        await signus.encrypt(wallet_handle, pool_handle + 1, my_did, trustee_did, message)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code
