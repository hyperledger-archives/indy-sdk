from indy import ledger

import json
import pytest

from tests.ledger.test_submit_request import ensure_previous_request_applied


@pytest.mark.asyncio
async def test_get_response_metadata_works_for_nym_requests(pool_handle, wallet_handle, identity_trustee1,
                                                            identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_ver_key, None, None)
    nym_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)
    response_metadata = json.loads(await ledger.get_response_metadata(nym_response))
    assert "seqNo" in response_metadata
    assert "txnTime" in response_metadata
    assert "lastTxnTime" not in response_metadata
    assert "lastSeqNo" not in response_metadata

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)
    get_nym_response = await ensure_previous_request_applied(pool_handle, get_nym_request,
                                                             lambda response: response['result']['data'] is not None)
    response_metadata = json.loads(await ledger.get_response_metadata(get_nym_response))
    assert "seqNo" in response_metadata
    assert "txnTime" in response_metadata
    assert "lastTxnTime" in response_metadata
    assert "lastSeqNo" not in response_metadata
