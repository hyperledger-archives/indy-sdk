import pytest

from indy import error
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
