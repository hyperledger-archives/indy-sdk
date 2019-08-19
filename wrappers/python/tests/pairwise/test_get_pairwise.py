import pytest
import json

from indy import error, pairwise


@pytest.mark.asyncio
async def test_get_pairwise_works(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)

    assert {'my_did': my_did} == json.loads(await pairwise.get_pairwise(wallet_handle, their_did))


@pytest.mark.asyncio
async def test_get_pairwise_works_with_metadata(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    metadata = 'some metadata'
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, metadata)

    assert {'my_did': my_did, 'metadata': metadata} == json.loads(await pairwise.get_pairwise(wallet_handle, their_did))


@pytest.mark.asyncio
async def test_get_pairwise_works_for_not_created_pairwise(wallet_handle, identity_trustee1):
    (their_did, _) = identity_trustee1

    with pytest.raises(error.WalletItemNotFound):
        await pairwise.get_pairwise(wallet_handle, their_did)
