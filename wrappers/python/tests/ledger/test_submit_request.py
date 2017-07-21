from tests.utils import pool, storage
from indy import ledger
from indy.pool import close_pool_ledger

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.fixture
async def pool_handle():
    handle = await pool.create_and_open_pool_ledger("pool_name")
    yield handle
    await close_pool_ledger(handle)


@pytest.mark.asyncio
async def test_submit_request_works(pool_handle):
    request = {
        "reqId": 1491566332010860,
        "identifier": "Th7MpTaRZVRYnPiabds81Y",
        "operation": {
            "type": "105",
            "dest": "Th7MpTaRZVRYnPiabds81Y"
        },
        "signature": "4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV"
    }

    expected_response = {
        "result": {
            "reqId": 1491566332010860,
            "identifier": "Th7MpTaRZVRYnPiabds81Y",
            "dest": "Th7MpTaRZVRYnPiabds81Y",
            "data": "{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\""
                    ",\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}",
            "type": "105",
        },
        "op": "REPLY"
    }
    response = json.loads((await ledger.submit_request(pool_handle, json.dumps(request))).decode())
    assert response == expected_response
