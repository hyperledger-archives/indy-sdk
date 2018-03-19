from indy.anoncreds import prover_get_credentials
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, "{}"))

    assert len(credentials) == 3


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_filter_by_issuer_did(wallet_handle, prepopulated_wallet, issuer_did):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, json.dumps({"issuer_did": issuer_did})))

    assert len(credentials) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_filter_by_schema_id(wallet_handle, prepopulated_wallet, gvt_schema_id):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, json.dumps({"schema_id": gvt_schema_id})))

    assert len(credentials) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_filter_by_schema_name(wallet_handle, prepopulated_wallet):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, json.dumps({"schema_name": "gvt"})))

    assert len(credentials) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_filter_by_cred_def_id(wallet_handle, prepopulated_wallet,
                                                                      issuer_1_xyz_cred_def_id):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, json.dumps({"cred_def_id": issuer_1_xyz_cred_def_id})))

    assert len(credentials) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_empty_result(wallet_handle, prepopulated_wallet, issuer_did):
    credentials = json.loads(
        await prover_get_credentials(
            wallet_handle, json.dumps({"issuer_did": issuer_did + 'a'})))

    assert len(credentials) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_credentials(invalid_wallet_handle, '{}')

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
