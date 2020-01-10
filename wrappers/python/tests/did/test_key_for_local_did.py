import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_key_for_local_did_works(wallet_handle, identity_trustee1):
    (_did, verkey) = identity_trustee1
    received_key = await did.key_for_local_did(wallet_handle, _did)
    assert verkey == received_key


@pytest.mark.asyncio
async def test_key_for_local_did_works_for_unknown_did(wallet_handle, did_my2):
    with pytest.raises(error.WalletItemNotFound):
        await did.key_for_local_did(wallet_handle, did_my2)
