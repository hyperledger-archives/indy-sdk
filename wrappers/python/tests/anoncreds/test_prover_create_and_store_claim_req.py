from indy import wallet
from indy.anoncreds import prover_create_and_store_claim_req
from indy.error import ErrorCode, IndyError

from tests.utils import storage, anoncreds
from tests.utils.wallet import create_and_open_wallet

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.fixture
async def wallet_handle_and_claim_def():
    handle = await create_and_open_wallet()
    claim_def = await anoncreds.prepare_common_wallet(handle)
    yield (handle, claim_def)
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works(wallet_handle_and_claim_def):
    schema_seq_no = 1
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, schema_seq_no)
    claim_req_json = json.loads(await prover_create_and_store_claim_req(wallet_handle_and_claim_def[0], prover_did,
                                                                        json.dumps(claim_offer_json),
                                                                        wallet_handle_and_claim_def[1],
                                                                        anoncreds.COMMON_MASTER_SECRET_NAME))
    assert claim_req_json['schema_seq_no'] == schema_seq_no
    assert claim_req_json['issuer_did'] == anoncreds.ISSUER_DID
    assert len(claim_req_json['blinded_ms']['u']) > 0


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_invalid_wallet(wallet_handle_and_claim_def):
    schema_seq_no = 1
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    invalid_wallet_handle = wallet_handle_and_claim_def[0] + 100
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, schema_seq_no)

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(invalid_wallet_handle, prover_did,
                                                json.dumps(claim_offer_json),
                                                wallet_handle_and_claim_def[1],
                                                anoncreds.COMMON_MASTER_SECRET_NAME)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_claim_def_does_not_correspond_offer_different_issuer_did(wallet_handle_and_claim_def):
    schema_seq_no = 1
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e3", schema_seq_no)

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(wallet_handle_and_claim_def[0], prover_did,
                                                json.dumps(claim_offer_json),
                                                wallet_handle_and_claim_def[1],
                                                anoncreds.COMMON_MASTER_SECRET_NAME)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_and_store_claim_req_works_for_claim_def_does_not_correspond_offer_different_schema_seq_no(wallet_handle_and_claim_def):
    schema_seq_no = 2
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, schema_seq_no)

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_claim_req(wallet_handle_and_claim_def[0], prover_did,
                                                json.dumps(claim_offer_json),
                                                wallet_handle_and_claim_def[1],
                                                anoncreds.COMMON_MASTER_SECRET_NAME)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code
