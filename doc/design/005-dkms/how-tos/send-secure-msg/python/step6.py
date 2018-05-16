async def read(wallet_handle, my_vk):
    with open('encrypted.dat', 'rb') as f:
        encrypted = f.read()
    decrypted = await crypto.auth_decrypt(wallet_handle, my_vk, encrypted)
    print(decrypted)
