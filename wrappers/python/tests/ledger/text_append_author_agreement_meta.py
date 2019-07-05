from indy import ledger

import json
import pytest


TEXT = 'some agreement text'
VERSION = '1.0.0'
ACCEPTANCE_MECH_TYPE = 'acceptance type 1'
TAA_DIGEST = '050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e'
TIME_OF_ACCEPTANCE = 123379200
REQUEST = json.dumps({
    "reqId": 1496822211362017764,
    "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
    "operation": {
        "type": "1",
        "dest": "VsKV7grR1BUE29mG2Fm2kX",
        "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
    }
})


def _check_request_acceptance_data(request: str):
    expected_meta = {
        "mechanism": ACCEPTANCE_MECH_TYPE,
        "taaDigest": TAA_DIGEST,
        "time": TIME_OF_ACCEPTANCE
    }

    request = json.loads(request)
    assert request['taaAcceptance'] == expected_meta


@pytest.mark.asyncio
async def test_append_txn_author_agreement_acceptance_to_request_works_for_text_version():
    request = await ledger.append_txn_author_agreement_acceptance_to_request(REQUEST, TEXT, VERSION, None,
                                                                             ACCEPTANCE_MECH_TYPE, TIME_OF_ACCEPTANCE)
    _check_request_acceptance_data(request)


@pytest.mark.asyncio
async def test_append_txn_author_agreement_acceptance_to_request_works_for_hash():
    request = await ledger.append_txn_author_agreement_acceptance_to_request(REQUEST, None, None, TAA_DIGEST,
                                                                             ACCEPTANCE_MECH_TYPE, TIME_OF_ACCEPTANCE)
    _check_request_acceptance_data(request)


@pytest.mark.asyncio
async def test_append_txn_author_agreement_acceptance_to_request_works_for_text_version_and_hash():
    request = await ledger.append_txn_author_agreement_acceptance_to_request(REQUEST, TEXT, VERSION, TAA_DIGEST,
                                                                             ACCEPTANCE_MECH_TYPE, TIME_OF_ACCEPTANCE)
    _check_request_acceptance_data(request)
