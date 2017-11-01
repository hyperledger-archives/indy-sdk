import asyncio

import json

import pytest

from indy import IndyError, ledger, signus, wallet
from indy.error import ErrorCode

signature = bytes(
    [20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190, 90, 60, 142, 23, 16,
     240, 189, 129, 45, 148, 245, 8, 102, 95, 95, 249, 100, 89, 41, 227, 213, 25, 100, 1, 232, 188, 245, 235, 186, 21,
     52, 176, 236, 11, 99, 70, 155, 159, 89, 215, 197, 239, 138, 5])


@pytest.mark.asyncio
async def test_verify_works_for_verkey_cached_in_wallet(wallet_handle, identity_trustee1, message):
    (my_did, my_verkey) = identity_trustee1

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, "verkey": my_verkey}))

    valid = await signus.verify_signature(wallet_handle, -1, my_did, message, signature)
    assert valid


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_runtime_config", ['{"freshness_time":1}'])
async def test_verify_works_for_expired_nym(wallet_handle, pool_handle, identity_trustee1,
                                            wallet_runtime_config, message):
    (my_did, my_verkey) = identity_trustee1

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, 'verkey': my_verkey}))

    await asyncio.sleep(2)

    valid = await signus.verify_signature(wallet_handle, pool_handle, my_did, message, signature)
    assert valid


@pytest.mark.asyncio
async def test_verify_works_for_invalid_wallet(pool_handle, wallet_handle, identity_trustee1, message):
    (my_did, _) = identity_trustee1

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle + 1, pool_handle, my_did, message, signature)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_invalid_pool(pool_handle, wallet_handle, did_trustee, message):
    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle + 1, did_trustee, message, signature)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_other_signer(pool_handle, wallet_handle, identity_steward1, message):
    (steward_did, steward_verkey) = identity_steward1
    await signus.store_their_did(wallet_handle, json.dumps({"did": steward_did, "verkey": steward_verkey}))

    valid = await signus.verify_signature(wallet_handle, pool_handle, steward_did, message, signature)
    assert not valid


@pytest.mark.asyncio
async def test_verify_works_for_get_nym_from_ledger_with_incompatible_wallet(pool_name, wallet_name, pool_handle,
                                                                             did_my1, message):
    pool_name = "other_" + pool_name
    wallet_name = "other_" + wallet_name

    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle, did_my1, message, signature)
    assert ErrorCode.WalletIncompatiblePoolError == e.value.error_code

    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet(wallet_name, None)
