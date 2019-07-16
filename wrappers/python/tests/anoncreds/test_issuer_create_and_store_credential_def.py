import json

import pytest

from indy.anoncreds import issuer_create_and_store_credential_def
from indy import error


@pytest.mark.asyncio
async def test_issuer_create_and_store_credential_def_works(wallet_handle, prepopulated_wallet):
    pass


@pytest.mark.asyncio
async def test_issuer_create_and_store_credential_def_works_for_invalid_wallet(wallet_handle, issuer_did,
                                                                               gvt_schema_json, tag,
                                                                               default_cred_def_config):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(error.WalletInvalidHandle):
        await issuer_create_and_store_credential_def(invalid_wallet_handle, issuer_did, gvt_schema_json, tag, "CL",
                                                     default_cred_def_config)



@pytest.mark.asyncio
async def test_issuer_create_and_store_credential_def_works_for_duplicate(wallet_handle, prepopulated_wallet,
                                                                          issuer_did, gvt_schema, tag,
                                                                          default_cred_def_config):
    with pytest.raises(error.AnoncredsCredDefAlreadyExistsError):
        await issuer_create_and_store_credential_def(wallet_handle, issuer_did, json.dumps(gvt_schema), tag, "CL",
                                                     default_cred_def_config)
