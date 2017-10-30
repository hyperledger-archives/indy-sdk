from indy import IndyError
from indy import signus, ledger
from indy.error import ErrorCode

import pytest
import json

schema_data = json.dumps({
    "name": "gvt2",
    "version": "2.0",
    "attr_names": ["name", "male"]
})


@pytest.mark.asyncio
async def test_replace_keys_apply_works(pool_handle, wallet_handle, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = await signus.create_and_store_my_did(wallet_handle, "{}")

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    new_verkey = await signus.replace_keys_start(wallet_handle, my_did, "{}")

    nym_request = await ledger.build_nym_request(my_did, my_did, new_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, nym_request)

    schema_request = await ledger.build_schema_request(my_did, schema_data)

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    assert ErrorCode.LedgerInvalidTransaction == e.value.error_code

    await signus.replace_keys_apply(wallet_handle, my_did)

    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)


@pytest.mark.asyncio
async def test_replace_keys_without_nym_transaction(pool_handle, wallet_handle, identity_trustee1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_verkey) = await signus.create_and_store_my_did(wallet_handle, "{}")

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    await signus.replace_keys_start(wallet_handle, my_did, "{}")
    await signus.replace_keys_apply(wallet_handle, my_did)

    schema_request = await ledger.build_schema_request(my_did, schema_data)

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    assert ErrorCode.LedgerInvalidTransaction == e.value.error_code
