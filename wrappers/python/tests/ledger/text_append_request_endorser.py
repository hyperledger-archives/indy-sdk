from indy import ledger

import json
import pytest


REQUEST = json.dumps({
    "reqId": 1496822211362017764,
    "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
    "operation": {
        "type": "1",
        "dest": "VsKV7grR1BUE29mG2Fm2kX",
        "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
    }
})


@pytest.mark.asyncio
async def test_append_request_endorser_works(did_trustee):
    request = await ledger.append_request_endorser(REQUEST, did_trustee)
    request = json.loads(request)
    assert did_trustee == request['endorser']
