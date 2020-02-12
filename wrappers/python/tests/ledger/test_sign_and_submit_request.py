import pytest

from indy import ledger, error


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

    with pytest.raises(error.PoolLedgerInvalidPoolHandle):
        await ledger.sign_and_submit_request(invalid_pool_handle, wallet_handle, trustee_did,
                                             nym_request)


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_wallet_handle(wallet_handle, pool_handle, identity_trustee1,
                                                                       identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    invalid_wallet_handle = wallet_handle + 1

    with pytest.raises(error.WalletInvalidHandle):
        await ledger.sign_and_submit_request(pool_handle, invalid_wallet_handle, trustee_did,
                                             nym_request)
