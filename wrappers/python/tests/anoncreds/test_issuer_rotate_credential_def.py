import json

import pytest

from indy.anoncreds import \
    issuer_create_and_store_credential_def, \
    issuer_rotate_credential_def_start, \
    issuer_rotate_credential_def_apply


@pytest.mark.asyncio
async def test_rotate_credential_def_works(wallet_handle, issuer_did, gvt_schema_json, tag):
    cred_def_id, cred_def_json = \
        await issuer_create_and_store_credential_def(wallet_handle, issuer_did, gvt_schema_json, "test_rotate_credential_def_works", "CL", None)

    temp_cred_def_json = await issuer_rotate_credential_def_start(wallet_handle, cred_def_id, None)

    assert not json.loads(cred_def_json) == json.loads(temp_cred_def_json)

    await issuer_rotate_credential_def_apply(wallet_handle, cred_def_id)
