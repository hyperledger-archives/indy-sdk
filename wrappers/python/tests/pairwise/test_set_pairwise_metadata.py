import pytest
import json

from indy import pairwise, error


@pytest.mark.asyncio
async def test_set_pairwise_metadata_works(wallet_handle, identity_my2, identity_trustee1, metadata):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1

    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)
    pairwise_without_metadata = await pairwise.get_pairwise(wallet_handle, their_did)

    await pairwise.set_pairwise_metadata(wallet_handle, their_did, metadata)
    pairwise_with_metadata = await pairwise.get_pairwise(wallet_handle, their_did)

    assert pairwise_without_metadata != pairwise_with_metadata
    assert {'my_did': my_did, 'metadata': metadata} == json.loads(pairwise_with_metadata)


@pytest.mark.asyncio
async def test_set_pairwise_metadata_works_for_not_created_pairwise(wallet_handle, identity_trustee1, metadata):
    (their_did, _) = identity_trustee1
    with pytest.raises(error.WalletItemNotFound):
        await pairwise.set_pairwise_metadata(wallet_handle, their_did, metadata)