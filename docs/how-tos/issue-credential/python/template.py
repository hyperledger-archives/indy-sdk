"""
Example demonstrating Proof Verification.

First Issuer creates Claim Definition for existing Schema.
After that, it issues a Claim to Prover (as in issue_credential.py example)

Once Prover has successfully stored its Claim, it uses Proof Request that he
received, to get Claims which satisfy the Proof Request from his wallet.
Prover uses the output to create Proof, using its Master Secret.
After that, Proof is verified against the Proof Request
"""

import asyncio
import json
import pprint

from indy import pool, ledger, wallet, did, anoncreds
from indy.error import ErrorCode, IndyError

from utils import get_pool_genesis_txn_path, PROTOCOL_VERSION

pool_name = 'pool'
issuer_wallet_config = json.dumps({"id": "issuer_wallet"})
issuer_wallet_credentials = json.dumps({"key": "issuer_wallet_key"})
genesis_file_path = get_pool_genesis_txn_path(pool_name)

def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))

async def proof_negotiation():
    try:
        await pool.set_protocol_version(PROTOCOL_VERSION)

        # Step 2 code goes here.

        # Step 3 code goes here.

        # Step 4 code goes here.

        # Step 5 code goes here.

    except IndyError as e:
        print('Error occurred: %s' % e)

def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(proof_negotiation())
    loop.close()

if __name__ == '__main__':
    main()