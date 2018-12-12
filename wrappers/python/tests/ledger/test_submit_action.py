import json

import pytest

from indy import ledger


@pytest.mark.asyncio
async def test_submit_action_works(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1

    get_validator_info_request = await ledger.build_get_validator_info_request(my_did)
    get_validator_info_request = await ledger.sign_request(wallet_handle, my_did, get_validator_info_request)
    await ledger.submit_action(pool_handle, get_validator_info_request, None, None)


@pytest.mark.asyncio
async def test_submit_action_works_for_nodes(pool_handle, wallet_handle, identity_my1):
    (my_did, _) = identity_my1
    nodes = ['Node1', 'Node2']

    get_validator_info_request = await ledger.build_get_validator_info_request(my_did)
    get_validator_info_request = await ledger.sign_request(wallet_handle, my_did, get_validator_info_request)
    response = json.loads(
        await ledger.submit_action(pool_handle, get_validator_info_request, json.dumps(nodes), None))
    assert 2 == len(response)
    assert 'Node1' in response
    assert 'Node2' in response
