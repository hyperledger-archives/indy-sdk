import pytest

from indy.anoncreds import prover_create_master_secret
from indy import error


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_create_master_secret_works(wallet_handle, prepopulated_wallet):
    pass


@pytest.mark.asyncio
async def test_prover_create_master_secret_works_invalid_wallet_handle(wallet_handle):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(error.WalletInvalidHandle):
        await prover_create_master_secret(invalid_wallet_handle, "master_secret_name")
