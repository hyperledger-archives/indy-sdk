from indy.anoncreds import prover_store_claim_offer
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_store_claim_offer_works(wallet_handle, prepopulated_wallet):
    _, claim_offer, _, _ = prepopulated_wallet

    await prover_store_claim_offer(wallet_handle, claim_offer)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_store_claim_offer_works_for_invalid_json(wallet_handle, prepopulated_wallet, issuer_did):
    claim_offer = {"issuer_did": issuer_did}

    with pytest.raises(IndyError) as e:
        await prover_store_claim_offer(wallet_handle, json.dumps(claim_offer))

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_store_claim_offer_works_for_invalid_wallet(wallet_handle, prepopulated_wallet):
    _, claim_offer, _, _ = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_store_claim_offer(invalid_wallet_handle, claim_offer)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
