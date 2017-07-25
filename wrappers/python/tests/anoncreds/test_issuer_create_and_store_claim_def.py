from indy import wallet
from indy.anoncreds import issuer_create_and_store_claim_def

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
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_issuer_create_and_store_claim_def_works(wallet_handle):
    schema = anoncreds.get_gvt_schema_json(1)
    claim_def_json = json.loads(await issuer_create_and_store_claim_def(wallet_handle, anoncreds.ISSUER_DID,
                                                             json.dumps(schema), "CL", False))
    assert len(claim_def_json['data']['primary']['r']) == 4
    assert len(claim_def_json['data']['primary']['n']) > 0
    assert len(claim_def_json['data']['primary']['s']) > 0
    assert len(claim_def_json['data']['primary']['rms']) > 0
    assert len(claim_def_json['data']['primary']['z']) > 0
    assert len(claim_def_json['data']['primary']['rctxt']) > 0
