import pytest

from indy.anoncreds import issuer_create_claim
from indy.error import ErrorCode, IndyError


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_claim_works(wallet_handle, prepopulated_wallet, gvt_claim_json):
    _, _, claim_req_json, _ = prepopulated_wallet
    await issuer_create_claim(wallet_handle, claim_req_json, gvt_claim_json, -1)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_claim_works_for_claim_does_not_correspond_to_claim_req(
        wallet_handle, prepopulated_wallet, xyz_claim_json):
    _, _, claim_req_json, _ = prepopulated_wallet

    with pytest.raises(IndyError) as e:
        await issuer_create_claim(wallet_handle, claim_req_json, xyz_claim_json, -1)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_claim_works_for_for_invalid_wallet_handle(
        wallet_handle, prepopulated_wallet, gvt_claim_json):
    _, _, claim_req_json, _ = prepopulated_wallet

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await issuer_create_claim(invalid_wallet_handle, claim_req_json, gvt_claim_json, -1)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
