import asyncio
import time

from src import anoncreds, crypto, ledger, getting_started


async def main():
    await anoncreds.demo()
    await crypto.demo()
    await ledger.demo()
    await getting_started.run()

if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    time.sleep(1)  # FIXME waiting for libindy thread complete
