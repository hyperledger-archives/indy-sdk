from indy.anoncreds import prover_get_credential

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credential_works(wallet_handle, prepopulated_wallet, id_credential_1, gvt_schema_id,
                                           issuer_1_gvt_cred_def_id):
    credential = json.loads(
        await prover_get_credential(wallet_handle, id_credential_1))

    assert credential['schema_id'] == gvt_schema_id
    assert credential['cred_def_id'] == issuer_1_gvt_cred_def_id
