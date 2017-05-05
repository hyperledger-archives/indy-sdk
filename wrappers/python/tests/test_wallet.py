import asyncio

from sovrin import *

async def main():
    await Wallet.create_wallet(None, "wallet", "wallet-type", "config", "creds")


loop = asyncio.get_event_loop()
loop.run_until_complete(main())
loop.close()
