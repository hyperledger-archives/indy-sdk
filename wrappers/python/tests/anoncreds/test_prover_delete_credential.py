import json
import pytest

from indy.anoncreds import prover_delete_credential, prover_get_credential, prover_store_credential
from indy import error

# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_delete_credential_works(wallet_handle, prepopulated_wallet):
    invalid_wallet_handle = wallet_handle + 100
    id_credential_x = 'id_credential_x';

    with pytest.raises(error.WalletInvalidHandle):
        await prover_delete_credential(invalid_wallet_handle, id_credential_x)

    # Store credential x
    await prover_store_credential(wallet_handle,
                                            id_credential_x,
                                            prepopulated_wallet[3],  # issuer_1_gvt_cred_req_metadata
                                            prepopulated_wallet[4],  # issuer_1_gvt_cred
                                            prepopulated_wallet[0],  # issuer_1_gvt_credential_def_json
                                            None)

    await prover_get_credential(wallet_handle, id_credential_x)  # raises exception if absent

    # Delete credential x and check that it's gone
    await prover_delete_credential(wallet_handle, id_credential_x)

    with pytest.raises(error.WalletItemNotFound):
        await prover_get_credential(wallet_handle, id_credential_x)  # make sure it's gone

    with pytest.raises(error.WalletItemNotFound):
        await prover_delete_credential(wallet_handle, id_credential_x)  # exercise double deletion
