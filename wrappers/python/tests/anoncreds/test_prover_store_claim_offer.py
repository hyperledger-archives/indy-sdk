from indy.anoncreds import prover_store_claim_offer
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_prover_store_claim_offer_works(init_common_wallet):
    claim_offer = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    await prover_store_claim_offer(init_common_wallet[0], json.dumps(claim_offer))


@pytest.mark.asyncio
async def test_prover_store_claim_offer_works_for_invalid_json(init_common_wallet):
    claim_offer = {"issuer_did": anoncreds.ISSUER_DID}

    try:
        await prover_store_claim_offer(init_common_wallet[0], json.dumps(claim_offer))
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) and \
               IndyError(ErrorCode.CommonInvalidStructure).args == e.args


@pytest.mark.asyncio
async def test_prover_store_claim_offer_works_for_invalid_wallet(init_common_wallet):
    claim_offer = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    invalid_wallet_handle = init_common_wallet[0] + 100

    try:
        await prover_store_claim_offer(invalid_wallet_handle, json.dumps(claim_offer))
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.WalletInvalidHandle)) == type(e) and \
               IndyError(ErrorCode.WalletInvalidHandle).args == e.args
