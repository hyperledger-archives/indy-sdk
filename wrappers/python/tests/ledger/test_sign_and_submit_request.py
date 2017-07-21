from tests.utils import pool, storage
from tests.utils.wallet import create_and_open_wallet
from indy import wallet, signus, ledger
from indy.pool import close_pool_ledger
from indy.error import ErrorCode, IndyError
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.fixture
async def pool_handle():
    handle = await pool.create_and_open_pool_ledger("pool_1")
    yield handle
    await close_pool_ledger(handle)


@pytest.fixture
async def wallet_handle():
    handle = await create_and_open_wallet()
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_sign_and_submit_request_works(wallet_handle, pool_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did.decode(), my_did.decode(), None, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did.decode(), nym_request.decode())


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_pool_handle(wallet_handle, pool_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did.decode(), my_did.decode(), None, None, None)
    invalid_pool_handle = pool_handle + 1

    try:
        await ledger.sign_and_submit_request(invalid_pool_handle, wallet_handle, trustee_did.decode(),
                                             nym_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.PoolLedgerInvalidPoolHandle)) == type(e) and \
               IndyError(ErrorCode.PoolLedgerInvalidPoolHandle).args == e.args


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_invalid_wallet_handle(wallet_handle, pool_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did.decode(), my_did.decode(), None, None, None)
    invalid_wallet_handle = wallet_handle + 1

    try:
        await ledger.sign_and_submit_request(pool_handle, invalid_wallet_handle, trustee_did.decode(),
                                             nym_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.WalletInvalidHandle)) == type(e) and \
               IndyError(ErrorCode.WalletInvalidHandle).args == e.args


@pytest.mark.asyncio
async def test_sign_and_submit_request_works_for_incompatible_wallet_and_pool(pool_handle):
    wallet_handle = await create_and_open_wallet(pool_name="pool_2")
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did.decode(), my_did.decode(), None, None, None)

    try:
        await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did.decode(),
                                             nym_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.WalletIncompatiblePoolError)) == type(e) and \
               IndyError(ErrorCode.WalletIncompatiblePoolError).args == e.args
    await wallet.close_wallet(wallet_handle)
