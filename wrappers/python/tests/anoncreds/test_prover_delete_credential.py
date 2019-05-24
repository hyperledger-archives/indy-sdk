from indy.anoncreds import prover_delete_credential, prover_get_credential
from indy.error import IndyError, ErrorCode

import json
import pytest

# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_delete_credential_works(wallet_handle, prepopulated_wallet, id_credential_x):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_delete_credential(invalid_wallet_handle, id_credential_x)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code

    await prover_get_credential(wallet_handle, id_credential_x)  # raises exception if absent

    await prover_delete_credential(wallet_handle, id_credential_x)

    with pytest.raises(IndyError) as e:
        await prover_get_credential(wallet_handle, id_credential_x)  # make sure it's gone
    assert ErrorCode.WalletItemNotFound == e.value.error_code

    with pytest.raises(IndyError) as e:
        await prover_delete_credential(wallet_handle, id_credential_x)  # exercise double deletion
    assert ErrorCode.WalletItemNotFound == e.value.error_code
