import pytest

from indy import wallet, error
from indy.anoncreds import issuer_create_credential


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_credential_works(wallet_handle, prepopulated_wallet):
    pass


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_credential_works_for_credential_values_not_correspond_to_credential_req(
        wallet_handle, prepopulated_wallet, xyz_cred_values_json):
    _, cred_offer, cred_req, _, _ = prepopulated_wallet

    with pytest.raises(error.CommonInvalidStructure):
        await issuer_create_credential(wallet_handle, cred_offer, cred_req, xyz_cred_values_json, None, None)



# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_issuer_create_credential_works_for_for_invalid_wallet_handle(
        wallet_handle, prepopulated_wallet, gvt_cred_values_json):
    _, cred_offer, cred_req, _, _ = prepopulated_wallet

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(error.WalletInvalidHandle):
        await issuer_create_credential(invalid_wallet_handle, cred_offer, cred_req, gvt_cred_values_json, None, None)
