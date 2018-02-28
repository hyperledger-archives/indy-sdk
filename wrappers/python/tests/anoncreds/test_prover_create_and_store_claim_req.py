from indy.anoncreds import prover_create_and_store_claim_req
from indy.error import ErrorCode, IndyError

import json
import pytest


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works(wallet_handle, prepopulated_wallet, issuer_did, prover_did,
                                                       schema_key, master_secret_name):
    claim_def_json, claim_offer, _, _ = prepopulated_wallet

    claim_req = json.loads(await prover_create_and_store_claim_req(wallet_handle, prover_did,
                                                                   claim_offer,
                                                                   claim_def_json,
                                                                   master_secret_name))
    assert claim_req['schema_key'] == schema_key
    assert claim_req['issuer_did'] == issuer_did
    assert len(claim_req['blinded_ms']['u']) > 0


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_invalid_wallet(wallet_handle, prepopulated_wallet,
                                                                          prover_did, master_secret_name):
    claim_def_json, claim_offer, _, _ = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(invalid_wallet_handle, prover_did,
                                                claim_offer,
                                                claim_def_json,
                                                master_secret_name)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_claim_def_does_not_correspond_offer_different_issuer_did(
        wallet_handle, prepopulated_wallet, prover_did, claim_offer_issuer_2_schema_1_json, master_secret_name):
    claim_def_json, _, _, _ = prepopulated_wallet

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(wallet_handle, prover_did,
                                                claim_offer_issuer_2_schema_1_json,
                                                claim_def_json,
                                                master_secret_name)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code
