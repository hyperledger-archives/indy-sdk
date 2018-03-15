import pytest

from indy.anoncreds import issuer_create_credential
from indy.error import ErrorCode, IndyError


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_credential_works(wallet_handle, prepopulated_wallet, gvt_credential_json):
    _, _, credential_req_json, _ = prepopulated_wallet
    await issuer_create_credential(wallet_handle, credential_req_json, gvt_credential_json, None, None, -1)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_credential_works_for_credential_does_not_correspond_to_credential_req(
        wallet_handle, prepopulated_wallet, xyz_credential_json):
    _, _, credential_req_json, _ = prepopulated_wallet

    with pytest.raises(IndyError) as e:
        await issuer_create_credential(wallet_handle, credential_req_json, xyz_credential_json, None, None, -1)

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_credential_works_for_for_invalid_wallet_handle(
        wallet_handle, prepopulated_wallet, gvt_credential_json):
    _, _, credential_req_json, _ = prepopulated_wallet

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await issuer_create_credential(invalid_wallet_handle, credential_req_json, gvt_credential_json, None, None, -1)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
