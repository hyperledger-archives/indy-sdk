import sys
import asyncio
import json
import random
from ctypes import cdll, CDLL
from time import sleep

import logging

from indy import wallet
from indy.error import ErrorCode, IndyError

from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision, vcx_messages_download
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

# logging.basicConfig(level=logging.DEBUG) uncomment to get logs

provisionConfig = {
    'agency_url': 'http://localhost:8080',
    'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
    'agency_verkey': 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
    'wallet_name': 'alice_wallet_' + str(random.randint(100, 999)),
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1'
}

if len(sys.argv) > 1 and sys.argv[1] == '--postgres':
    # load postgres dll and configure postgres wallet
    print("Initializing postgres wallet")
    stg_lib = cdll.LoadLibrary("libindystrgpostgres.dylib")
    result = stg_lib.postgresstorage_init()
    if result != 0:
        print("Error unable to load postgres wallet storage", result)
        sys.exit(0)

    provisionConfig['wallet_type'] = 'postgres_storage'
    provisionConfig['storage_config'] = '{"url":"localhost:5432"}'
    provisionConfig['storage_credentials'] = '{"account":"postgres","password":"mysecretpassword","admin_account":"postgres","admin_password":"mysecretpassword"}'

    print("Success, loaded postgres wallet storage")


async def main():
    if len(sys.argv) > 1 and sys.argv[1] == '--postgres':
        # create wallet in advance
        print("Provision postgres wallet in advance")
        wallet_config = {
            'id': provisionConfig['wallet_name'],
            'storage_type': provisionConfig['wallet_type'],
            'storage_config': json.loads(provisionConfig['storage_config']),
        }
        wallet_creds = {
            'key': provisionConfig['wallet_key'],
            'storage_credentials': json.loads(provisionConfig['storage_credentials']),
        }
        try:
            await wallet.create_wallet(json.dumps(wallet_config), json.dumps(wallet_creds))
        except IndyError as ex:
            if ex.error_code == ErrorCode.PoolLedgerConfigAlreadyExistsError:
                pass
        print("Postgres wallet provisioned")

    payment_plugin = cdll.LoadLibrary("libnullpay.dylib")
    payment_plugin.nullpay_init()

    handled_offers = []
    handled_requests = []

    print("#7 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to alice
    config['institution_name'] = 'alice'
    config['institution_logo_url'] = 'http://robohash.org/456'
    config['genesis_path'] = 'docker.txn'

    print("#8 Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))

    print("#9 Input faber.py invitation details")
    details = input('invite details: ')

    print("#10 Convert to valid json and string and create a connection to faber")
    jdetails = json.loads(details)
    connection_to_faber = await Connection.create_with_details('faber', json.dumps(jdetails))
    await connection_to_faber.connect('{"use_public_did": true}')
    await connection_to_faber.update_state()

    print("Serialize connection")
    connection_data = await connection_to_faber.serialize()
    connection_to_faber.release()
    connection_to_faber = None
    print(connection_data)

    option = input('Poll messages? [Y/n] ')
    while option != 'N' and option != 'n':
        print("Deserialize connection")
        my_connection = await Connection.deserialize(connection_data)
        sleep(2)

        await handle_messages(my_connection, handled_offers, handled_requests)

        sleep(2)
        print("Serialize connection")
        connection_data = await my_connection.serialize()
        my_connection.release()
        my_connection = None

        option = input('Poll messages? [Y/n] ')

    print("Done, pause before exiting program")
    sleep(2)


async def handle_messages(my_connection, handled_offers, handled_requests):
    print("Check for and handle offers")
    offers = await Credential.get_offers(my_connection)

    for offer in offers:
        handled = False
        for handled_offer in handled_offers:
            if offer[0]['msg_ref_id'] == handled_offer['msg_ref_id']:
                handled = True
                break
        if not handled:
            save_offer = offer[0].copy()
            print(" >>> handling offer", save_offer['msg_ref_id'])
            await handle_credential_offer(my_connection, offer)
            handled_offers.append(save_offer)

    print("Check for and handle proof requests")
    requests = await DisclosedProof.get_requests(my_connection)
    for request in requests:
        print("request", type(request), request)
        handled = False
        for handled_request in handled_requests:
            if request['msg_ref_id'] == handled_request['msg_ref_id']:
                handled = True
                break
        if not handled:
            save_request = request.copy()
            print(" >>> handling proof", save_request['msg_ref_id'])
            await handle_proof_request(my_connection, request)
            handled_requests.append(save_request)


async def handle_credential_offer(my_connection, offer):
    print("Handling offer ...")

    print("Create a credential object from the credential offer")
    credential = await Credential.create('credential', offer)

    print("#15 After receiving credential offer, send credential request")
    await credential.send_request(my_connection, 0)

    # serialize/deserialize credential - wait for Faber to send credential
    credential_data = await credential.serialize()

    while True:
        print("#16 Poll agency and accept credential offer from faber")
        my_credential = await Credential.deserialize(credential_data)
        await my_credential.update_state()
        credential_state = await my_credential.get_state()
        if credential_state == State.Accepted:
            break
        else:
            credential_data = await my_credential.serialize()
            sleep(2)

    print("Accepted")


async def handle_proof_request(my_connection, request):
    print("Handling proof request ...")

    print("#23 Create a Disclosed proof object from proof request")
    proof = await DisclosedProof.create('proof', request)

    print("#24 Query for credentials in the wallet that satisfy the proof request")
    credentials = await proof.get_creds()

    # TODO list credentials and let Alice select
    # Use the first available credentials to satisfy the proof request
    for attr in credentials['attrs']:
        credentials['attrs'][attr] = {
            'credential': credentials['attrs'][attr][0]
        }

    print("#25 Generate the proof")
    await proof.generate_proof(credentials, {})

    # TODO figure out why this always segfaults
    print("#26 Send the proof to faber")
    await proof.send_proof(my_connection)

    # serialize/deserialize proof
    proof_data = await proof.serialize()

    while True:
        print("#27 Poll agency and wait for faber to accept proof")
        my_proof = await DisclosedProof.deserialize(proof_data)
        await my_proof.update_state()
        proof_state = await my_proof.get_state()
        if proof_state == State.Accepted:
            break
        else:
            proof_data = await my_proof.serialize()
            sleep(2)

    print("proof_state", proof_state)

    print("Sent")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())

