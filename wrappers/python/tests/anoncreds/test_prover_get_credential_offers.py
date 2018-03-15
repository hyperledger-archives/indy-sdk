from indy.anoncreds import prover_get_credential_offers
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    credential_offers = json.loads(
        await prover_get_credential_offers(wallet_handle, "{}"))

    assert len(credential_offers) == 3


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_filter_by_issuer(wallet_handle, prepopulated_wallet, issuer_did,
                                                                       issuer_1_gvt_cred_def_id,
                                                                       issuer_1_xyz_cred_def_id):
    credential_offers = json.loads(
        await prover_get_credential_offers(wallet_handle, json.dumps({"issuer_did": issuer_did})))

    assert len(credential_offers) == 2
    credential_offers = credential_offers_info(credential_offers)
    assert {"issuer_did": issuer_did, "cred_def_id": issuer_1_gvt_cred_def_id} in credential_offers
    assert {"issuer_did": issuer_did, "cred_def_id": issuer_1_xyz_cred_def_id} in credential_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_filter_by_schema_name(wallet_handle, prepopulated_wallet,
                                                                            issuer_did, prover_did,
                                                                            issuer_1_xyz_cred_def_id):
    credential_offers = json.loads(
        await prover_get_credential_offers(
            wallet_handle, json.dumps({"schema_name": "xyz"})))

    assert len(credential_offers) == 1
    credential_offers = credential_offers_info(credential_offers)
    assert {'issuer_did': issuer_did, 'cred_def_id': issuer_1_xyz_cred_def_id} in credential_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_filter_by_schema_id(wallet_handle, prepopulated_wallet,
                                                                          issuer_did, prover_did, xyz_schema_id,
                                                                          issuer_1_xyz_cred_def_id):
    credential_offers = json.loads(
        await prover_get_credential_offers(
            wallet_handle, json.dumps({"schema_id": xyz_schema_id})))

    assert len(credential_offers) == 1
    credential_offers = credential_offers_info(credential_offers)
    assert {'issuer_did': issuer_did, 'cred_def_id': issuer_1_xyz_cred_def_id} in credential_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_filter_by_cred_def_id(wallet_handle, prepopulated_wallet,
                                                                            issuer_did, issuer_1_xyz_cred_def_id):
    credential_offers = json.loads(
        await prover_get_credential_offers(wallet_handle, json.dumps({"cred_def_id": issuer_1_xyz_cred_def_id})))

    assert len(credential_offers) == 1
    credential_offers = credential_offers_info(credential_offers)
    assert {'issuer_did': issuer_did, 'cred_def_id': issuer_1_xyz_cred_def_id} in credential_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_no_results(wallet_handle, prepopulated_wallet, issuer_did):
    credential_offers = json.loads(
        await prover_get_credential_offers(wallet_handle, json.dumps({"issuer_did": issuer_did + 'a'})))

    assert len(credential_offers) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_offers_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_credential_offers(invalid_wallet_handle, "{}")

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


def credential_offers_info(credential_offers):
    return [{"issuer_did": credential_offer['issuer_did'], "cred_def_id": credential_offer['cred_def_id']}
            for credential_offer in credential_offers]
