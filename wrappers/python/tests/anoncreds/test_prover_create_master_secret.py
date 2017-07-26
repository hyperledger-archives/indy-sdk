from indy.anoncreds import prover_create_master_secret
from indy.error import ErrorCode, IndyError

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_prover_create_master_secret_works(init_common_wallet):
    await prover_create_master_secret(init_common_wallet[0], "master_secret_name")


@pytest.mark.asyncio
async def test_prover_create_master_secret_works_invalid_wallet_handle(init_common_wallet):
    invalid_wallet_handle = init_common_wallet[0] + 100

    try:
        await prover_create_master_secret(invalid_wallet_handle, "master_secret_name")
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.WalletInvalidHandle)) == type(e) and \
               IndyError(ErrorCode.WalletInvalidHandle).args == e.args

