import json
import pytest

from indy import ledger, anoncreds, non_secrets


@pytest.mark.asyncio
async def test_get_schema_works(pool_handle, wallet_handle, identity_my):
    (my_did, my_ver_key) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))

    schema_request = await ledger.build_schema_request(my_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    options_json = {
        "noCache": False,
        "noUpdate": False,
        "noStore": False,
        "minFresh": -1,
    }

    await non_secrets.get_schema(pool_handle, wallet_handle, my_did, schema_id, json.dumps(options_json))
