from indy.anoncreds import issuer_create_claim
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_issuer_create_claim_works(init_common_wallet):
    claim_req = anoncreds.get_claim_req()
    claim_json = anoncreds.get_gvt_claim_json()
    (_, claim_json) = await issuer_create_claim(init_common_wallet[0], json.dumps(claim_req), json.dumps(claim_json), -1, -1)


@pytest.mark.asyncio
async def test_issuer_create_claim_works_for_claim_does_not_correspond_to_claim_req(init_common_wallet):
    claim_req = anoncreds.get_claim_req()
    claim_json = anoncreds.get_xyz_claim_json()
    with pytest.raises(IndyError) as e:
        await issuer_create_claim(init_common_wallet[0], json.dumps(claim_req), json.dumps(claim_json), -1, -1)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_issuer_create_claim_works_for_for_invalid_wallet_handle(init_common_wallet):
    claim_req = anoncreds.get_claim_req()
    claim_json = anoncreds.get_gvt_claim_json()
    invalid_wallet_handle = init_common_wallet[0] + 100
    with pytest.raises(IndyError) as e:
        await issuer_create_claim(invalid_wallet_handle, json.dumps(claim_req), json.dumps(claim_json), -1, -1)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
