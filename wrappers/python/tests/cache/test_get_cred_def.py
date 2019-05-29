import json

import logging
import pytest
import time

from indy import ledger, anoncreds, cache, IndyError
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_get_cred_def_works(pool_handle, wallet_handle, identity_my):
    (my_did, my_ver_key) = identity_my
    options_json = {
        "noCache": False,
        "noUpdate": False,
        "noStore": False,
        "minFresh": -1,
    }

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))

    schema_request = await ledger.build_schema_request(my_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    # retry if previous request is not applied
    for _ in range(3):
        try:
            schema_json = await cache.get_schema(pool_handle, wallet_handle, my_did, schema_id, json.dumps(options_json))
        except IndyError as err:
            if err.error_code == ErrorCode.LedgerNotFound:
                logger = logging.getLogger(__name__)
                logger.warning(err)
                time.sleep(5)
            else:
                raise err

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, my_did, schema_json, "TAG", "CL",
                                                               json.dumps({"support_revocation": False}))

    cred_def_request = await ledger.build_cred_def_request(my_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, cred_def_request)

    # retry if previous request is not applied
    for _ in range(3):
        try:
            await cache.get_cred_def(pool_handle, wallet_handle, my_did, cred_def_id, json.dumps(options_json))
        except IndyError as err:
            if err.error_code == ErrorCode.LedgerNotFound:
                logger = logging.getLogger(__name__)
                logger.warning(err)
                time.sleep(5)
            else:
                raise err
