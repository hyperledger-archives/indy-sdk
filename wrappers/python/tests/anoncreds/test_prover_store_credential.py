import pytest

from indy.anoncreds import prover_store_credential
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_prover_store_credential_works(wallet_handle, prepopulated_wallet):
    _, _, _, credential_json = prepopulated_wallet

    await prover_store_credential(wallet_handle, "id_1", credential_json, None)


@pytest.mark.asyncio
async def test_prover_store_credential_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    _, _, _, credential_json = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_store_credential(invalid_wallet_handle, "id_1", credential_json, None)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
