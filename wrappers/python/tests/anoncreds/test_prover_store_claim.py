import pytest

from indy.anoncreds import prover_store_claim
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_prover_store_claim_works(wallet_handle, prepopulated_wallet):
    _, _, _, claim_json = prepopulated_wallet

    await prover_store_claim(wallet_handle, claim_json, None)


@pytest.mark.asyncio
async def test_prover_store_claim_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    _, _, _, claim_json = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_store_claim(invalid_wallet_handle, claim_json, None)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
