from indy.anoncreds import prover_store_credential_offer
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_store_credential_offer_works(wallet_handle, prepopulated_wallet):
    _, credential_offer, _, _ = prepopulated_wallet

    await prover_store_credential_offer(wallet_handle, credential_offer)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_store_credential_offer_works_for_invalid_json(wallet_handle, prepopulated_wallet, issuer_did):
    credential_offer = {"issuer_did": issuer_did}

    with pytest.raises(IndyError) as e:
        await prover_store_credential_offer(wallet_handle, json.dumps(credential_offer))

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_store_credential_offer_works_for_invalid_wallet(wallet_handle, prepopulated_wallet):
    _, credential_offer, _, _ = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_store_credential_offer(invalid_wallet_handle, credential_offer)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
