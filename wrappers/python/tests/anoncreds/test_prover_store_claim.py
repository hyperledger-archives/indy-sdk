import pytest

from indy.anoncreds import prover_create_and_store_claim_req, prover_store_claim, \
    prover_create_master_secret, issuer_create_claim
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_prover_store_claim_works(wallet_handle, prepopulated_wallet, prover_did,
                                        gvt_claim_json,
                                        claim_offer_issuer_1_schema_1_json,
                                        master_secret_name_1):
    claim_def_json, = prepopulated_wallet
    await prover_create_master_secret(wallet_handle, master_secret_name_1)

    claim_req = await prover_create_and_store_claim_req(wallet_handle, prover_did,
                                                        claim_offer_issuer_1_schema_1_json, claim_def_json,
                                                        master_secret_name_1)

    (_, claim_json) = await issuer_create_claim(wallet_handle, claim_req, gvt_claim_json, -1)

    await prover_store_claim(wallet_handle, claim_json, None)


@pytest.mark.asyncio
async def test_prover_store_claim_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet, prover_did,
                                                                  gvt_claim_json,
                                                                  claim_offer_issuer_1_schema_1_json,
                                                                  master_secret_name_2):
    claim_def_json, = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    await prover_create_master_secret(wallet_handle, master_secret_name_2)

    claim_req = await prover_create_and_store_claim_req(wallet_handle, prover_did,
                                                        claim_offer_issuer_1_schema_1_json, claim_def_json,
                                                        master_secret_name_2)

    (_, claim_json) = await issuer_create_claim(wallet_handle, claim_req, gvt_claim_json, -1)

    with pytest.raises(IndyError) as e:
        await prover_store_claim(invalid_wallet_handle, claim_json, None)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
