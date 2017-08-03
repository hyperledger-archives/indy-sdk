from indy_sdk.anoncreds import prover_get_claims
from indy_sdk.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_filter(init_common_wallet):
    claims = json.loads(await prover_get_claims(init_common_wallet[0], "{}"))
    assert len(claims) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did(init_common_wallet):
    claims = json.loads(await prover_get_claims(init_common_wallet[0], '{{"issuer_did":"{}"}}'.format(anoncreds.ISSUER_DID)))
    assert len(claims) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did_and_schema_seq_no(init_common_wallet):
    claims = json.loads(await prover_get_claims(init_common_wallet[0], json.dumps(anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1))))
    assert len(claims) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_result(init_common_wallet):
    claims = json.loads(await prover_get_claims(init_common_wallet[0], '{"schema_seq_no":10}'))
    assert len(claims) == 0


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_invalid_wallet_handle(init_common_wallet):
    invalid_wallet_handle = init_common_wallet[0] + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claims(invalid_wallet_handle, '{}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
