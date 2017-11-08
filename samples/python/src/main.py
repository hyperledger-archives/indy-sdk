import asyncio
import time

from src import anoncreds, signus, ledger, agent


async def main():
    await anoncreds.demo()
    await signus.demo()
    await ledger.demo()
    await agent.demo()

if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    time.sleep(1)  # FIXME waiting for libindy thread complete
