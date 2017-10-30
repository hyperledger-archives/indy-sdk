import asyncio

import json

import pytest

from indy import IndyError, ledger, signus, wallet
from indy.error import ErrorCode

signature = bytes(
    [169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94,
     204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48,
     30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11])


@pytest.mark.asyncio
async def test_verify_works_for_verkey_cached_in_wallet(pool_handle, wallet_handle, identity_my1, message):
    (my_did, my__verkey) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, "verkey": my__verkey}))

    valid = await signus.verify_signature(wallet_handle, pool_handle, my_did, message, signature)
    assert valid


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_runtime_config", ['{"freshness_time":1}'])
async def test_verify_works_for_expired_nym(wallet_handle, pool_handle, identity_trustee1, identity_my1,
                                            wallet_runtime_config, message):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    await signus.store_their_did(wallet_handle, json.dumps({"did": my_did, 'verkey': my_verkey}))

    await asyncio.sleep(2)

    valid = await signus.verify_signature(wallet_handle, pool_handle, my_did, message, signature)
    assert valid


@pytest.mark.asyncio
async def test_verify_works_for_invalid_wallet(pool_handle, wallet_handle, identity_my1, message):
    (my_did, _) = identity_my1

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle + 1, pool_handle, my_did, message, signature)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_invalid_pool(pool_handle, wallet_handle, identity_my1, message):
    (my_did, _) = identity_my1

    with pytest.raises(IndyError) as e:
        await signus.verify_signature(wallet_handle, pool_handle + 1, my_did, message, signature)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_verify_works_for_other_signer(pool_handle, wallet_handle, identity_steward1, identity_my1,
                                             identity_trustee1, message):
    (steward_did, steward_verkey) = identity_steward1
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    await signus.store_their_did(wallet_handle, json.dumps({"did": steward_did, "verkey": steward_verkey}))

    local_signature = await signus.sign(wallet_handle, trustee_did, message)

    valid = await signus.verify_signature(wallet_handle, pool_handle, steward_did, message, local_signature)
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
