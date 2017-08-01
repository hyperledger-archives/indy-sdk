from indy.anoncreds import prover_create_and_store_claim_req
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works(init_common_wallet):
    schema_seq_no = 1
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, schema_seq_no)
    claim_req_json = json.loads(await prover_create_and_store_claim_req(init_common_wallet[0], prover_did,
                                                                        json.dumps(claim_offer_json),
                                                                        init_common_wallet[1],
                                                                        anoncreds.COMMON_MASTER_SECRET_NAME))
    assert claim_req_json['schema_seq_no'] == schema_seq_no
    assert claim_req_json['issuer_did'] == anoncreds.ISSUER_DID
    assert len(claim_req_json['blinded_ms']['u']) > 0


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_invalid_wallet(init_common_wallet):
    schema_seq_no = 1
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    invalid_wallet_handle = init_common_wallet[0] + 100
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, schema_seq_no)

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(invalid_wallet_handle, prover_did,
                                                json.dumps(claim_offer_json),
                                                init_common_wallet[1],
                                                anoncreds.COMMON_MASTER_SECRET_NAME)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_claim_def_does_not_correspond_offer_different_issuer_did(init_common_wallet):
    schema_seq_no = 1
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e3", schema_seq_no)

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(init_common_wallet[0], prover_did,
                                                json.dumps(claim_offer_json),
                                                init_common_wallet[1],
                                                anoncreds.COMMON_MASTER_SECRET_NAME)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_claim_def_does_not_correspond_offer_different_schema_seq_no(init_common_wallet):
    schema_seq_no = 2
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, schema_seq_no)

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(init_common_wallet[0], prover_did,
                                                json.dumps(claim_offer_json),
                                                init_common_wallet[1],
                                                anoncreds.COMMON_MASTER_SECRET_NAME)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code
