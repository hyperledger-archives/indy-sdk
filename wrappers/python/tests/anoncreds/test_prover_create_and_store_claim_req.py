from indy.anoncreds import prover_create_and_store_credential_req
from indy.error import ErrorCode, IndyError

import json
import pytest


@pytest.mark.asyncio
async def test_prover_create_and_store_credential_req_works(wallet_handle, prepopulated_wallet, prover_did,
                                                            master_secret_name):
    credential_def_json, credential_offer, _, _ = prepopulated_wallet

    json.loads(await prover_create_and_store_credential_req(wallet_handle, prover_did,
                                                            credential_offer,
                                                            credential_def_json,
                                                            master_secret_name))


@pytest.mark.asyncio
async def test_prover_create_and_store_credential_req_works_for_invalid_wallet(wallet_handle, prepopulated_wallet,
                                                                               prover_did, master_secret_name):
    credential_def_json, credential_offer, _, _ = prepopulated_wallet
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_create_and_store_credential_req(invalid_wallet_handle, prover_did,
                                                     credential_offer,
                                                     credential_def_json,
                                                     master_secret_name)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
