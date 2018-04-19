from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_revoc_reg_def_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    data = {
        "ver": "1.0",
        "id": "RevocRegID",
        "revocDefType": "CL_ACCUM",
        "tag": "TAG1",
        "credDefId": "CredDefID",
        "value": {
            "issuanceType": "ISSUANCE_ON_DEMAND",
            "maxCredNum": 5,
            "tailsHash": "s",
            "tailsLocation": "http://tails.location.com",
            "publicKeys": {
                "accumKey": {
                    "z": "1111 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                }
            }
        }
    }

    expected_response = {
        "operation": {
            "credDefId": data["credDefId"],
            "id": data["id"],
            "revocDefType": data["revocDefType"],
            "tag": data["tag"],
            "type": "113",
            "value": data["value"]
        }
    }

    request = json.loads(await ledger.build_revoc_reg_def_request(identifier, json.dumps(data)))
    assert expected_response.items() <= request.items()
