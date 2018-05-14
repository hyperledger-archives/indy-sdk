import asyncio
import json
import pprint

from indy import pool, ledger, wallet, signus, anoncreds
from indy.error import IndyError


pool_name = 'pool'
wallet_name = 'wallet'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'


def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def save_schema_and_cred_def():
    try:
        # Step 2 code goes here.

        # Step 3 code goes here.

		# Step 4 code goes here.

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(save_schema_and_cred_def())
    loop.close()


if __name__ == '__main__':
    main()

