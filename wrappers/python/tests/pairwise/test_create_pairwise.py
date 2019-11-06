import pytest

from indy import  error, pairwise

UNKNOWN_DID = 'NcYxiDXkpYi6ov5FcYDi1e'


@pytest.mark.asyncio
async def test_create_pairwise_works(wallet_handle, identity_my2, identity_trustee1, metadata):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, metadata)


@pytest.mark.asyncio
async def test_create_pairwise_works_for_not_found_my_did(wallet_handle, identity_trustee1):
    (their_did, _) = identity_trustee1
    with pytest.raises(error.WalletItemNotFound):
        await pairwise.create_pairwise(wallet_handle, their_did, UNKNOWN_DID, None)