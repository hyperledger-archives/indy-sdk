async def prep(wallet_handle, my_vk, their_vk, msg):
    with open('plaintext.txt', 'rb') as f:
        msg = f.read()
    encrypted = await crypto.auth_crypt(wallet_handle, my_vk, their_vk, msg)
    print('encrypted = %s' % repr(encrypted))
    with open('encrypted.dat', 'wb') as f:
        f.write(bytes(encrypted))
    print('prepping %s' % msg)

