from tests.utils import storage
from indy import ledger

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_build_get_claim_def_request_works():
    identifier = "identifier"
    _ref = 1
    signature_type = "signature_type"
    origin = "origin"

    expected_response = {
        "identifier": "identifier",
        "operation": {
            "type": "108",
            "ref": 1,
            "signature_type": "signature_type",
            "origin": "origin"
        }
    }

    response = json.loads((await ledger.build_get_claim_def_txn(
        identifier, _ref, signature_type, origin
    )).decode())
    assert expected_response.items() <= response.items()
