from indy import IndyError
from indy import pairwise

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_is_pairwise_exists_works(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)
    assert await pairwise.is_pairwise_exists(wallet_handle, their_did)


@pytest.mark.asyncio
async def test_is_pairwise_exists_works_for_not_created(wallet_handle, identity_my2, identity_trustee1):
    (their_did, _) = identity_trustee1
    assert not await pairwise.is_pairwise_exists(wallet_handle, their_did)


@pytest.mark.asyncio
async def test_is_pairwise_exists_works_for_invalid_handle(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)

    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await pairwise.is_pairwise_exists(invalid_wallet_handle, their_did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code