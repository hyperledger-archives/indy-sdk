from indy.anoncreds import prover_store_claim_offer
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest


@pytest.mark.asyncio
async def test_prover_store_claim_offer_works(init_common_wallet):
    claim_offer = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    await prover_store_claim_offer(init_common_wallet[0], json.dumps(claim_offer))


@pytest.mark.asyncio
async def test_prover_store_claim_offer_works_for_invalid_json(init_common_wallet):
    claim_offer = {"issuer_did": anoncreds.ISSUER_DID}

    with pytest.raises(IndyError) as e:
        await prover_store_claim_offer(init_common_wallet[0], json.dumps(claim_offer))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_prover_store_claim_offer_works_for_invalid_wallet(init_common_wallet):
    claim_offer = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    invalid_wallet_handle = init_common_wallet[0] + 100

    with pytest.raises(IndyError) as e:
        await prover_store_claim_offer(invalid_wallet_handle, json.dumps(claim_offer))
    assert ErrorCode.WalletInvalidHandle == e.value.error_code

