import json

import pytest

from indy.anoncreds import issuer_create_and_store_claim_def
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_issuer_create_and_store_claim_def_works(wallet_handle, issuer_did, gvt_schema_json):
    claim_def = json.loads(await issuer_create_and_store_claim_def(wallet_handle, issuer_did, gvt_schema_json, "CL", False))

    assert len(claim_def['data']['primary']['r']) == 4
    assert len(claim_def['data']['primary']['n']) > 0
    assert len(claim_def['data']['primary']['s']) > 0
    assert len(claim_def['data']['primary']['rms']) > 0
    assert len(claim_def['data']['primary']['z']) > 0
    assert len(claim_def['data']['primary']['rctxt']) > 0


@pytest.mark.asyncio
async def test_issuer_create_and_store_claim_def_works_for_invalid_wallet(wallet_handle, issuer_did, gvt_schema_json):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await issuer_create_and_store_claim_def(
            invalid_wallet_handle, issuer_did, gvt_schema_json, None, False)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_issuer_create_and_store_claim_def_works_for_duplicate(wallet_handle, issuer_did, gvt_schema_json):
    await issuer_create_and_store_claim_def(wallet_handle, issuer_did, gvt_schema_json, "CL", False)

    with pytest.raises(IndyError) as e:
        await issuer_create_and_store_claim_def(wallet_handle, issuer_did, gvt_schema_json, "CL", False)

    assert ErrorCode.AnoncredsClaimDefAlreadyExistsError == e.value.error_code
