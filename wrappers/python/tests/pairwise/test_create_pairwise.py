from indy import IndyError
from indy import did
from indy import pairwise

import pytest
import json

from indy.error import ErrorCode

UNKNOWN_DID = 'NcYxiDXkpYi6ov5FcYDi1e'


@pytest.mark.asyncio
async def test_create_pairwise_works(wallet_handle, identity_my2, identity_trustee1, metadata):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, metadata)


@pytest.mark.asyncio
async def test_create_pairwise_works_for_empty_metadata(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)


@pytest.mark.asyncio
async def test_create_pairwise_works_for_not_found_my_did(wallet_handle, identity_trustee1):
    (their_did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await pairwise.create_pairwise(wallet_handle, their_did, UNKNOWN_DID, None)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_create_pairwise_works_for_not_found_their_did(wallet_handle, identity_trustee1):
    (my_did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await pairwise.create_pairwise(wallet_handle, UNKNOWN_DID, my_did, None)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_create_pairwise_works_for_invalid_handle(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1

    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await pairwise.create_pairwise(invalid_wallet_handle, their_did, my_did, None)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
