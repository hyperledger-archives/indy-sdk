from indy import wallet

async def create_and_open_wallet(pool_name="pool_type_1", wallet_name="wallet_type_1"):
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    assert wallet_handle is not None
    return wallet_handle


async def close_wallet(wallet_handle):
    await wallet.close_wallet(wallet_handle)
