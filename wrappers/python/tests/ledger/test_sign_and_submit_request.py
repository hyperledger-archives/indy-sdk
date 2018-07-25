import json

from indy import wallet, did, ledger
from indy.error import ErrorCode, IndyError
import pytest


@pytest.mark.asyncio
async def test_sign_and_submit_request_works(wallet_handle, pool_handle, identity_trustee1, identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_pool_handle(wallet_handle, pool_handle, identity_trustee1,
                                                                     identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    invalid_pool_handle = pool_handle + 1

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(invalid_pool_handle, wallet_handle, trustee_did,
                                             nym_request)

    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_wallet_handle(wallet_handle, pool_handle, identity_trustee1,
                                                                       identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    invalid_wallet_handle = wallet_handle + 1

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(pool_handle, invalid_wallet_handle, trustee_did,
                                             nym_request)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
