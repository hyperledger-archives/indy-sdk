import time

from src import anoncreds, anoncreds_revocation, crypto, ledger, getting_started
from src.utils import run_coroutine


async def main():
    await getting_started.run()
    await anoncreds.demo()
    await anoncreds_revocation.demo()
    await anoncreds.demo()
    await crypto.demo()
    await ledger.demo()

if __name__ == '__main__':
    run_coroutine(main)
    time.sleep(1)  # FIXME waiting for libindy thread complete
