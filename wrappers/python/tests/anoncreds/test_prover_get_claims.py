from indy import wallet
from indy.anoncreds import prover_get_claims
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
async def wallet_handle():
    handle = await create_and_open_wallet()
    await anoncreds.prepare_common_wallet(handle)
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_filter(wallet_handle):
    claims = json.loads(await prover_get_claims(wallet_handle, "{}"))
    assert len(claims) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did(wallet_handle):
    claims = json.loads(await prover_get_claims(wallet_handle, '{{"issuer_did":"{}"}}'.format(anoncreds.ISSUER_DID)))
    assert len(claims) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did_and_schema_seq_no(wallet_handle):
    claims = json.loads(await prover_get_claims(wallet_handle, json.dumps(anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1))))
    assert len(claims) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_result(wallet_handle):
    claims = json.loads(await prover_get_claims(wallet_handle, '{"schema_seq_no":10}'))
    assert len(claims) == 0


@pytest.mark.asyncio
async def test_prover_get_claims_works_for_invalid_wallet_handle(wallet_handle):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claims(invalid_wallet_handle, '{}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
