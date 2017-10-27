import json

from indy import IndyError
from indy import signus
from indy import wallet
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_key_for_did_works_for_my_did(pool_handle, wallet_handle, identity_trustee1):
    (did, verkey) = identity_trustee1
    received_key = await signus.key_for_did(pool_handle, wallet_handle, did)
    assert verkey == received_key


@pytest.mark.asyncio
async def test_key_for_did_works_for_their_did(pool_handle, wallet_handle, did_my1, verkey_my1):
    await signus.store_their_did(wallet_handle, json.dumps({'did': did_my1, 'verkey': verkey_my1}))
    received_key = await signus.key_for_did(pool_handle, wallet_handle, did_my1)
    assert verkey_my1 == received_key


@pytest.mark.asyncio
async def test_key_for_did_works_for_unknown_did(pool_handle, wallet_handle, did_my2):
    with pytest.raises(IndyError) as e:
        await signus.key_for_did(pool_handle, wallet_handle, did_my2)
    assert ErrorCode.CommonInvalidState == e.value.error_code


@pytest.mark.asyncio
async def test_key_for_did_works_for_incompatible_wallet_and_pool(pool_name, wallet_name, pool_handle, did_my1):
    pool_name = "other_" + pool_name
    wallet_name = "other_" + wallet_name
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    with pytest.raises(IndyError) as e:
        await signus.key_for_did(pool_handle, wallet_handle, did_my1)
    assert ErrorCode.WalletIncompatiblePoolError == e.value.error_code

    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet(wallet_name, None)


@pytest.mark.asyncio
async def test_key_for_did_works_for_invalid_pool_handle(pool_handle, wallet_handle, identity_trustee1):
    (did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        invalid_pool_handle = pool_handle + 1
        await signus.key_for_did(invalid_pool_handle, wallet_handle, did)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_key_for_did_works_for_invalid_wallet_handle(pool_handle, wallet_handle, identity_trustee1):
    (did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await signus.key_for_did(pool_handle, invalid_wallet_handle, did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
