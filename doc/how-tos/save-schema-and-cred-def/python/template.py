"""
Example demonstrating how to write Schema and Cred Definition on the ledger

As a setup, Steward (already on the ledger) adds Trust Anchor to the ledger.

After that, Steward builds the SCHEMA request to add new schema to the ledger.
Once that succeeds, Trust Anchor uses anonymous credentials to issue and store
claim definition for the Schema added by Steward.
"""


import asyncio
import json
import pprint

from indy import pool, ledger, wallet, did, anoncreds
from indy.error import IndyError


pool_name = 'pool'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'
wallet_config = json.dumps({"id": "wallet"})
wallet_credentials = json.dumps({"key": "wallet_key"})
PROTOCOL_VERSION=2

def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def write_schema_and_cred_def():
    try:
        await pool.set_protocol_version(PROTOCOL_VERSION)
        # Step 2 code goes here.

        # Step 3 code goes here.

        # Step 4 code goes here.

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(write_schema_and_cred_def())
    loop.close()


if __name__ == '__main__':
    main()


