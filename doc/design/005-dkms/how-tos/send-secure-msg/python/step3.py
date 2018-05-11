async def init():
    me = input('Who are you? ').strip()
    wallet_name = '%s-wallet' % me

    # 1. Create Wallet and Get Wallet Handle
    try:
        await wallet.create_wallet('pool1', wallet_name, None, None, None)
    except:
        pass
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    print('wallet = %s' % wallet_handle)

    (my_did, my_vk) = await did.create_and_store_my_did(wallet_handle, "{}")
    print('my_did and verkey = %s %s' % (my_did, my_vk))

    their = input("Other party's DID and verkey? ").strip().split(' ')
    return wallet_handle, my_did, my_vk, their[0], their[1]
