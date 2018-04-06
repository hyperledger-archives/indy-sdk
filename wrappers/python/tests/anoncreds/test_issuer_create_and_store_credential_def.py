import json

import pytest

from indy.anoncreds import issuer_create_and_store_credential_def
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_issuer_create_and_store_credential_def_works(wallet_handle, issuer_did, gvt_schema, tag,
                                                             default_cred_def_config):
    gvt_schema["id"] = "test_issuer_create_and_store_credential_def_works"

    (_, cred_def_json) = await issuer_create_and_store_credential_def(wallet_handle, issuer_did,
                                                                      json.dumps(gvt_schema), tag, "CL",
                                                                      default_cred_def_config)
    cred_def = json.loads(cred_def_json)

    assert cred_def['value']['primary']['r']
    assert cred_def['value']['primary']['n']
    assert cred_def['value']['primary']['s']
    assert cred_def['value']['primary']['rms']
    assert cred_def['value']['primary']['z']
    assert cred_def['value']['primary']['rctxt']


@pytest.mark.asyncio
async def test_issuer_create_and_store_credential_def_works_for_invalid_wallet(wallet_handle, issuer_did,
                                                                               gvt_schema_json, tag,
                                                                               default_cred_def_config):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await issuer_create_and_store_credential_def(invalid_wallet_handle, issuer_did, gvt_schema_json, tag, "CL",
                                                     default_cred_def_config)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_issuer_create_and_store_credential_def_works_for_duplicate(wallet_handle, issuer_did, gvt_schema, tag,
                                                                          default_cred_def_config):
    gvt_schema["id"] = "test_issuer_create_and_store_credential_def_works_for_duplicate"

    await issuer_create_and_store_credential_def(wallet_handle, issuer_did, json.dumps(gvt_schema), tag, "CL",
                                                 default_cred_def_config)

    with pytest.raises(IndyError) as e:
        await issuer_create_and_store_credential_def(wallet_handle, issuer_did, json.dumps(gvt_schema), tag, "CL",
                                                     default_cred_def_config)

    assert ErrorCode.AnoncredsCredDefAlreadyExistsError == e.value.error_code
