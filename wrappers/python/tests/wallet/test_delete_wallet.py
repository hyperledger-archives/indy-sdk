from indy import IndyError
from indy import wallet
from indy.error import ErrorCode
>>>>>>> master


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works(wallet_name, xwallet):
    await wallet.delete_wallet(wallet_name, None)


@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup, wallet_handle_cleanup", [(False, False)])
async def test_delete_wallet_works_for_closed(wallet_name, wallet_handle):
    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet(wallet_name, None)


<<<<<<< HEAD
@pytest.mark.skip(reason="There is BUG in indy_sdk")
async def test_delete_wallet_works_for_opened(cleanup_storage):
=======
# noinspection PyUnusedLocal
@pytest.mark.skip(reason="TODO: FIXME: Create a bug!!!")
@pytest.mark.asyncio
async def test_delete_wallet_works_for_opened(wallet_name, wallet_handle):
>>>>>>> master
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_name, None)

    assert ErrorCode.CommonIOError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works_for_twice(wallet_name, xwallet):
    await wallet.delete_wallet(wallet_name, None)

    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_name, None)

    assert ErrorCode.CommonIOError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_delete_wallet_works_for_not_created(wallet_name, path_home):
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_name, None)

    assert ErrorCode.CommonIOError == e.value.error_code
