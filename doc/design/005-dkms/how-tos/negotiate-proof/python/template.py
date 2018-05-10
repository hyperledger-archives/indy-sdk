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
import sys

sys.path.insert(0, '/home/vagrant/code/evernym/indy-sdk/wrappers/python')

from indy import pool, ledger, wallet, did, anoncreds, crypto
from indy.error import IndyError


seq_no = 1
pool_name = 'pool'
issuer_wallet_name = 'issuer_wallet'
prover_wallet_name = 'prover_wallet'
issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'
prover_did = 'VsKV7grR1BUE29mG2Fm2kX'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'

def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def proof_negotiation():
    try:
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

