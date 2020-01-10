import json
import time
from random import randint

from indy import ledger, did, wallet, pool
from src.utils import get_pool_genesis_txn_path, run_coroutine, PROTOCOL_VERSION
import logging


logger = logging.getLogger(__name__)


async def demo():
    logger.info("Transaction Author Agreement sample -> started")

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    # Trustee preparation
    trustee = {
        'seed': '000000000000000000000000Trustee1',
        'wallet_config': json.dumps({'id': 'trustee_wallet'}),
        'wallet_credentials': json.dumps({'key': 'trustee_wallet_key'}),
        'pool_name': 'trustee_pool',
    }

    # Trustee opens pool ledger
    trustee['genesis_txn_path'] = get_pool_genesis_txn_path(trustee['pool_name'])
    trustee['pool_config'] = json.dumps({"genesis_txn": str(trustee['genesis_txn_path'])})
    await pool.create_pool_ledger_config(trustee['pool_name'], trustee['pool_config'])

    trustee['pool'] = await pool.open_pool_ledger(trustee['pool_name'], None)

    #  Trustee creates and open Wallet
    await wallet.create_wallet(trustee['wallet_config'], trustee['wallet_credentials'])
    trustee['wallet'] = await wallet.open_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    #  Trustee creates DID (predefined on the ledger)
    (trustee['did'], trustee['verkey']) = \
        await did.create_and_store_my_did(trustee['wallet'], json.dumps({"seed": trustee['seed']}))

    # Trustee set up Transaction Author Agreement on the ledger
    trustee['txn_agreement'] = {
        'text': 'Indy transaction author agreement text',
        'version': str(randint(0, 1000)),
    }

    trustee['acceptance_mechanisms'] = {
        'aml': {
            'Demo acceptance': 'Demo acceptance mechanism'
        },
        'version': str(randint(0, 1000)),
        'context': 'Some common context for acceptance mechanisms'
    }

    # Trustee sends Acceptance Mechanisms to the ledger
    aml_req = await ledger.build_acceptance_mechanisms_request(trustee['did'],
                                                               json.dumps(trustee['acceptance_mechanisms']['aml']),
                                                               trustee['acceptance_mechanisms']['version'],
                                                               trustee['acceptance_mechanisms']['context'])
    await ledger.sign_and_submit_request(trustee['pool'], trustee['wallet'], trustee['did'], aml_req)

    # Trustee sends Transaction Agreement to the ledger
    txn_agreement_req = \
        await ledger.build_txn_author_agreement_request(trustee['did'],
                                                        trustee['txn_agreement']['text'],
                                                        trustee['txn_agreement']['version'])
    await ledger.sign_and_submit_request(trustee['pool'], trustee['wallet'], trustee['did'], txn_agreement_req)

    # User initialization
    user = {
        'wallet_config': json.dumps({'id': 'user_wallet'}),
        'wallet_credentials': json.dumps({'key': 'user_wallet_key'}),
        'pool_name': 'user_pool'
    }

    # User opens pool ledger
    user['genesis_txn_path'] = get_pool_genesis_txn_path(user['pool_name'])
    user['pool_config'] = json.dumps({"genesis_txn": str(user['genesis_txn_path'])})
    await pool.create_pool_ledger_config(user['pool_name'], user['pool_config'])

    user['pool'] = await pool.open_pool_ledger(user['pool_name'], None)

    # User opens pool ledger
    await wallet.create_wallet(user['wallet_config'], user['wallet_credentials'])
    user['wallet'] = await wallet.open_wallet(user['wallet_config'], user['wallet_credentials'])

    # User creates DID and tells it Trustee
    (user['did'], user['verkey']) = await did.create_and_store_my_did(user['wallet'], "{}")

    trustee['user_did'] = user['did']
    trustee['user_verkey'] = user['verkey']

    # Trustee sends NYM transaction for user
    nym_req = await ledger.build_nym_request(trustee['did'], trustee['user_did'], trustee['user_verkey'], None, None)

    #           Append Transaction Agreement to request
    nym_req = \
        await ledger.append_txn_author_agreement_acceptance_to_request(nym_req,
                                                                       trustee['txn_agreement']['text'],
                                                                       trustee['txn_agreement']['version'],
                                                                       None,
                                                                       next(iter(
                                                                           trustee['acceptance_mechanisms']['aml'])),
                                                                       int(time.time()))
    # Sends request
    await ledger.sign_and_submit_request(trustee['pool'], trustee['wallet'], trustee['did'], nym_req)

    # User sends ATTRIB transaction to the Ledger

    # User gets Transaction Agreement from the ledger
    get_txn_author_agreement_req = await ledger.build_get_txn_author_agreement_request(user['did'], None)
    get_txn_author_agreement_resp = json.loads(await ledger.submit_request(user['pool'], get_txn_author_agreement_req))
    user['txn_agreement'] = get_txn_author_agreement_resp['result']['data']

    # User gets Acceptance Mechanisms from the ledger
    get_acc_mech_req = await ledger.build_get_acceptance_mechanisms_request(user['did'], None, None)
    get_acc_mech_resp = json.loads(await ledger.submit_request(user['pool'], get_acc_mech_req))
    user['acceptance_mechanisms'] = get_acc_mech_resp['result']['data']

    #       User sends ATTRIB transaction to Ledger
    attr_req = \
        await ledger.build_attrib_request(user['did'], user['did'], None, '{"endpoint":{"ha":"127.0.0.1:5555"}}', None)

    #       Appends Transaction Agreement to request
    attr_req = \
        await ledger.append_txn_author_agreement_acceptance_to_request(attr_req, user['txn_agreement']['text'],
                                                                       user['txn_agreement']['version'],
                                                                       None,
                                                                       next(iter(user['acceptance_mechanisms']['aml'])),
                                                                       int(time.time()))

    #       Sends to request
    resp = json.loads(await ledger.sign_and_submit_request(user['pool'], user['wallet'], user['did'], attr_req))
    assert resp['op'] == 'REPLY'

    # Trustee reset Transaction Agreement
    txn_agreement_req = \
        await ledger.build_txn_author_agreement_request(trustee['did'], '', str(randint(0, 1000)))
    await ledger.sign_and_submit_request(trustee['pool'], trustee['wallet'], trustee['did'], txn_agreement_req)

    #       User sends ATTRIB transaction to Ledger without Transaction Agreement
    attr_req = \
        await ledger.build_attrib_request(user['did'], user['did'], None, '{"endpoint":{"ha":"10.0.0.2:5555"}}', None)
    resp = json.loads(await ledger.sign_and_submit_request(user['pool'], user['wallet'], user['did'], attr_req))
    assert resp['op'] == 'REPLY'

    #  Trustee closes and deletes Wallet
    await wallet.close_wallet(trustee['wallet'])
    await wallet.delete_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    #  User closes and deletes Wallet
    await wallet.close_wallet(user['wallet'])
    await wallet.delete_wallet(user['wallet_config'], user['wallet_credentials'])

    #  Trustee and USer closes and deletes Wallet
    await pool.close_pool_ledger(trustee['pool'])
    await pool.close_pool_ledger(user['pool'])

    # Delete pool ledger config
    await pool.delete_pool_ledger_config(trustee['pool_name'])
    await pool.delete_pool_ledger_config(user['pool_name'])

    logger.info("Transaction Author Agreement sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
