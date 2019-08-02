import json

import pytest

from indy import did, error



@pytest.mark.asyncio
async def test_key_for_did_works_for_my_did(wallet_handle, identity_trustee1):
    (_did, verkey) = identity_trustee1
    received_key = await did.key_for_did(-1, wallet_handle, _did)
    assert verkey == received_key


@pytest.mark.asyncio
async def test_key_for_did_works_for_their_did(wallet_handle, did_my1, verkey_my1):
    await did.store_their_did(wallet_handle, json.dumps({'did': did_my1, 'verkey': verkey_my1}))
    received_key = await did.key_for_did(-1, wallet_handle, did_my1)
    assert verkey_my1 == received_key


@pytest.mark.asyncio
async def test_key_for_did_works_for_unknown_did(pool_handle, wallet_handle, did_my2):
    with pytest.raises(error.WalletItemNotFound):
        await did.key_for_did(pool_handle, wallet_handle, did_my2)


@pytest.mark.asyncio
async def test_key_for_did_works_for_invalid_pool_handle(pool_handle, wallet_handle, did_my1):
    with pytest.raises(error.PoolLedgerInvalidPoolHandle):
        await did.key_for_did(pool_handle + 1, wallet_handle, did_my1)


@pytest.mark.asyncio
async def test_key_for_did_works_for_invalid_wallet_handle(wallet_handle, identity_trustee1):
    (_did, _) = identity_trustee1
    with pytest.raises(error.WalletInvalidHandle):
        invalid_wallet_handle = wallet_handle + 1
        await did.key_for_did(-1, invalid_wallet_handle, _did)
