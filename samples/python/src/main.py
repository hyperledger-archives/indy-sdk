import time

from src import anoncreds, crypto, ledger, getting_started
from src.utils import run_coroutine


async def main():
    await getting_started.run()

if __name__ == '__main__':
    run_coroutine(main)
    time.sleep(1)  # FIXME waiting for libindy thread complete
