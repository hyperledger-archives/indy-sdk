from indy.anoncreds import prover_create_and_store_claim_req, prover_store_claim,\
    prover_create_master_secret, issuer_create_claim
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest


@pytest.mark.asyncio
async def test_prover_store_claim_works(init_common_wallet):
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    await prover_create_master_secret(init_common_wallet[0], anoncreds.COMMON_MASTER_SECRET_NAME_1)
    claim_req = await prover_create_and_store_claim_req(init_common_wallet[0], prover_did,
                                                        json.dumps(claim_offer_json), init_common_wallet[1],
                                                        anoncreds.COMMON_MASTER_SECRET_NAME_1)

    (_, claim_json) = await issuer_create_claim(init_common_wallet[0], claim_req,
                                                json.dumps(anoncreds.get_gvt_claim_json()), -1, -1)
    await prover_store_claim(init_common_wallet[0], claim_json)


@pytest.mark.asyncio
async def test_prover_store_claim_works_for_invalid_wallet_handle(init_common_wallet):
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    invalid_wallet_handle = init_common_wallet[0] + 100
    await prover_create_master_secret(init_common_wallet[0], anoncreds.COMMON_MASTER_SECRET_NAME_2)
    claim_req = await prover_create_and_store_claim_req(init_common_wallet[0], prover_did,
                                                        json.dumps(claim_offer_json), init_common_wallet[1],
                                                        anoncreds.COMMON_MASTER_SECRET_NAME_2)

    (_, claim_json) = await issuer_create_claim(init_common_wallet[0], claim_req,
                                                json.dumps(anoncreds.get_gvt_claim_json()), -1, -1)

    with pytest.raises(IndyError) as e:
        await prover_store_claim(invalid_wallet_handle, claim_json)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
