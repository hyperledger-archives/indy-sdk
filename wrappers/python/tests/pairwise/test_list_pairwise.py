import pytest
import json

from indy import pairwise


@pytest.mark.asyncio
async def test_list_pairwise_works(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, None)

    list_pairwise = json.loads(await pairwise.list_pairwise(wallet_handle))
    assert 1 == len(list_pairwise)
    assert {"my_did": my_did, "their_did": their_did} == json.loads(list_pairwise[0])


@pytest.mark.asyncio
async def test_list_pairwise_works_for_empty_result(wallet_handle):
    list_pairwise = json.loads(await pairwise.list_pairwise(wallet_handle))
    assert 0 == len(list_pairwise)
