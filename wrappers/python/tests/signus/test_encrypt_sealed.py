import asyncio

import json

import pytest

from indy import IndyError, signus
from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'.encode('utf-8')


@pytest.mark.asyncio
async def test_encrypt_sealed_works_for_pk_cached_in_wallet(pool_handle, wallet_handle, identity_trustee1):
    (did, verkey) = identity_trustee1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": verkey}))
    await signus.encrypt_sealed(wallet_handle, pool_handle, did, message)


@pytest.mark.asyncio
async def test_encrypt_sealed_works_for_get_pk_from_ledger(pool_handle, wallet_handle, identity_my1):
    (did, _) = identity_my1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did}))
    await signus.encrypt_sealed(wallet_handle, pool_handle, did, message)


@pytest.mark.asyncio
async def test_encrypt_sealed_works_for_get_nym_from_ledger(pool_handle, wallet_handle, identity_my1):
    (did, _) = identity_my1
    await signus.encrypt_sealed(wallet_handle, pool_handle, did, message)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_runtime_config", ['{"freshness_time":1}'])
async def test_encrypt_sealed_works_for_expired_nym(wallet_handle, pool_handle, identity_my1, wallet_runtime_config):
    (did, verkey) = identity_my1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did, 'verkey': verkey}))

    await asyncio.sleep(2)

    await signus.encrypt_sealed(wallet_handle, pool_handle, did, message)


@pytest.mark.asyncio
async def test_encrypt_sealed_works_for_invalid_wallet_handle(wallet_handle, pool_handle, identity_trustee1):
    (did, verkey) = identity_trustee1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": verkey}))

    with pytest.raises(IndyError) as e:
        await signus.encrypt_sealed(wallet_handle + 1, pool_handle, did, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_encrypt_sealed_works_for_invalid_pool_handle(wallet_handle, pool_handle, identity_trustee1):
    (did, verkey) = identity_trustee1
    await signus.store_their_did(wallet_handle, json.dumps({"did": did, "verkey": verkey}))

    with pytest.raises(IndyError) as e:
        await signus.encrypt_sealed(wallet_handle, pool_handle + 1, did, message)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code
