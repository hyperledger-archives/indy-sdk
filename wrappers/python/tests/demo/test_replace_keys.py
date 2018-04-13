from indy import IndyError
from indy import anoncreds
from indy import did, ledger
from indy.error import ErrorCode

import pytest
import json


@pytest.mark.asyncio
async def test_replace_keys_apply_works(pool_handle, wallet_handle, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = await did.create_and_store_my_did(wallet_handle, "{}")

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    new_verkey = await did.replace_keys_start(wallet_handle, my_did, "{}")

    nym_request = await ledger.build_nym_request(my_did, my_did, new_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, nym_request)

    (_, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)

    response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    assert json.loads(response)['op'] == 'REQNACK'

    await did.replace_keys_apply(wallet_handle, my_did)

    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)


@pytest.mark.asyncio
async def test_replace_keys_without_nym_transaction(pool_handle, wallet_handle, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = await did.create_and_store_my_did(wallet_handle, "{}")

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    await did.replace_keys_start(wallet_handle, my_did, "{}")
    await did.replace_keys_apply(wallet_handle, my_did)

    (_, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)

    response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    assert json.loads(response)['op'] == 'REQNACK'

