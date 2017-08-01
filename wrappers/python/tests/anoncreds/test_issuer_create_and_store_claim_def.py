from indy.anoncreds import issuer_create_and_store_claim_def
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest


@pytest.mark.asyncio
async def test_issuer_create_and_store_claim_def_works(wallet_handle):
    schema = anoncreds.get_gvt_schema_json(1)
    claim_def_json = json.loads(await issuer_create_and_store_claim_def(wallet_handle, anoncreds.ISSUER_DID,
                                                             json.dumps(schema), "CL", False))
    assert len(claim_def_json['data']['primary']['r']) == 4
    assert len(claim_def_json['data']['primary']['n']) > 0
    assert len(claim_def_json['data']['primary']['s']) > 0
    assert len(claim_def_json['data']['primary']['rms']) > 0
    assert len(claim_def_json['data']['primary']['z']) > 0
    assert len(claim_def_json['data']['primary']['rctxt']) > 0


@pytest.mark.asyncio
async def test_issuer_create_and_store_claim_def_works_for_invalid_wallet(wallet_handle):
    schema = anoncreds.get_gvt_schema_json(1)
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await issuer_create_and_store_claim_def(
            invalid_wallet_handle, anoncreds.ISSUER_DID, json.dumps(schema), None, False)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
