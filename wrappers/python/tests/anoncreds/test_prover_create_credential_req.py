from indy.anoncreds import prover_create_credential_req
from indy.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_prover_create_credential_req_works(wallet_handle, prepopulated_wallet, prover_did, master_secret_id):
    credential_def_json, credential_offer, _, _, _ = prepopulated_wallet

    await prover_create_credential_req(wallet_handle, prover_did, credential_offer, credential_def_json,
                                       master_secret_id)


@pytest.mark.asyncio
async def test_prover_create_credential_req_works_for_invalid_wallet(wallet_handle, prepopulated_wallet,
                                                                     prover_did, master_secret_id):
    credential_def_json, credential_offer, _, _, _ = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_create_credential_req(invalid_wallet_handle, prover_did, credential_offer, credential_def_json,
                                           master_secret_id)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
