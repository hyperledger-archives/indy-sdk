from indy import IndyError
from indy import pairwise

import pytest
import json

from indy.error import ErrorCode


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
async def test_set_pairwise_metadata_works_for_reset(wallet_handle, identity_my2, identity_trustee1, metadata):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1

    await pairwise.create_pairwise(wallet_handle, their_did, my_did, metadata)
    pairwise_with_metadata = await pairwise.get_pairwise(wallet_handle, their_did)
    assert {'my_did': my_did, 'metadata': metadata} == json.loads(pairwise_with_metadata)

    await pairwise.set_pairwise_metadata(wallet_handle, their_did, None)
    pairwise_without_metadata = await pairwise.get_pairwise(wallet_handle, their_did)
    assert {'my_did': my_did} == json.loads(pairwise_without_metadata)
    assert pairwise_without_metadata != pairwise_with_metadata


@pytest.mark.asyncio
async def test_set_pairwise_metadata_works_for_not_created_pairwise(wallet_handle, identity_trustee1, metadata):
    (their_did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await pairwise.set_pairwise_metadata(wallet_handle, their_did, metadata)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_set_pairwise_metadata_works_for_invalid_handle(wallet_handle, identity_my2, identity_trustee1, metadata):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)

    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await pairwise.set_pairwise_metadata(invalid_wallet_handle, their_did, metadata)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
