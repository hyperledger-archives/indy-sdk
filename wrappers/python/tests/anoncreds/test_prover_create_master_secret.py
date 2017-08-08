from indy.anoncreds import prover_create_master_secret
from indy.error import ErrorCode, IndyError

import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_create_master_secret_works(wallet_handle, prepopulated_wallet):
    pass


@pytest.mark.asyncio
async def test_prover_create_master_secret_works_invalid_wallet_handle(wallet_handle):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_create_master_secret(invalid_wallet_handle, "master_secret_name")

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
