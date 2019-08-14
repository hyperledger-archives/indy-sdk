import asyncio
import time
import re

# Not used here, but will be required for the next steps
from indy import crypto, did, wallet

# Step 5 code goes here, replacing the prep() stub.
async def prep(wallet_handle, my_vk, their_vk, msg):
    print('prepping %s' % msg)

# Step 3 code goes here, replacing the init() stub.
async def init():
    return None, None, None, None, None

# Step 6 code goes here, replacing the read() stub.
async def read(wallet_handle, my_vk):
    print('reading')

async def demo():
    wallet_handle, my_did, my_vk, their_did, their_vk = await init()

    while True:
        argv = input('> ').strip().split(' ')
        cmd = argv[0].lower()
        rest = ' '.join(argv[1:])
        if re.match(cmd, 'prep'):
            await prep(wallet_handle, my_vk, their_vk, rest)
        elif re.match(cmd, 'read'):
            await read(wallet_handle, my_vk)
        elif re.match(cmd, 'quit'):
            break
        else:
            print('Huh?')

if __name__ == '__main__':
    try:
        loop = asyncio.get_event_loop()
        loop.run_until_complete(demo())
        time.sleep(1)  # waiting for libindy thread complete
    except KeyboardInterrupt:
        print('')
