from tests.utils.wallet import create_and_open_wallet
from indy import wallet, signus, ledger
from indy.error import ErrorCode, IndyError
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_sign_and_submit_request_works(wallet_handle, pool_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_pool_handle(wallet_handle, pool_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    invalid_pool_handle = pool_handle + 1

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(invalid_pool_handle, wallet_handle, trustee_did,
                                             nym_request)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_wallet_handle(wallet_handle, pool_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)
    invalid_wallet_handle = wallet_handle + 1

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(pool_handle, invalid_wallet_handle, trustee_did,
                                             nym_request)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_incompatible_wallet_and_pool(pool_handle):
    wallet_handle = await create_and_open_wallet(pool_name="pool_2")
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did, my_did, None, None, None)

    with pytest.raises(IndyError) as e:
        await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did,
                                             nym_request)
    assert ErrorCode.WalletIncompatiblePoolError == e.value.error_code

    await wallet.close_wallet(wallet_handle)
