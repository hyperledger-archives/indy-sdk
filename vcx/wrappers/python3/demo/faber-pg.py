import sys
import asyncio
import json
import random
from ctypes import cdll
from time import sleep

import logging

from indy import wallet
from indy.error import ErrorCode, IndyError

from vcx.api.connection import Connection
from vcx.api.credential_def import CredentialDef
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.proof import Proof
from vcx.api.schema import Schema
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State, ProofState

# logging.basicConfig(level=logging.DEBUG) uncomment to get logs

# 'agency_url': URL of the agency
# 'agency_did':  public DID of the agency
# 'agency_verkey': public verkey of the agency
# 'wallet_name': name for newly created encrypted wallet
# 'wallet_key': encryption key for encoding wallet
# 'payment_method': method that will be used for payments
provisionConfig = {
  'agency_url':'http://localhost:8080',
  'agency_did':'VsKV7grR1BUE29mG2Fm2kX',
  'agency_verkey':'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
  'wallet_name':'faber_wallet_' + str(random.randint(100, 999)),
  'wallet_key':'123',
  'payment_method': 'null',
  'enterprise_seed':'000000000000000000000000Trustee1'
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

    print("#1 Provision an agent and wallet, get back configuration details")
    print('provisionConfig', provisionConfig)
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to faber
    config['institution_name'] = 'Faber'
    config['institution_logo_url'] = 'http://robohash.org/234'
    config['genesis_path'] = 'docker.txn'
    
    print("#2 Initialize libvcx with new configuration")
    print('config', config)
    await vcx_init_with_config(json.dumps(config))

    print("#3 Create a new schema on the ledger")
    version = format("%d.%d.%d" % (random.randint(1, 101), random.randint(1, 101), random.randint(1, 101)))
    schema = await Schema.create('schema_uuid', 'degree schema', version, ['name', 'date', 'degree'], 0)
    schema_id = await schema.get_schema_id()

    print("#4 Create a new credential definition on the ledger")
    cred_def = await CredentialDef.create('credef_uuid', 'degree', schema_id, 0)
    cred_def_handle = cred_def.handle
    cred_def_id = await cred_def.get_cred_def_id()

    print("#5 Create a connection to alice and print out the invite details")
    connection_to_alice = await Connection.create('alice')
    await connection_to_alice.connect('{"use_public_did": true}')
    await connection_to_alice.update_state()
    details = await connection_to_alice.invite_details(False)
    print("**invite details**")
    print(json.dumps(details))
    print("******************")

    connection_data = await connection_to_alice.serialize()
    connection_to_alice.release()
    connection_to_alice = None
    print(connection_data)

    while True:
        print("#6 Poll agency and wait for alice to accept the invitation (start alice.py now)")
        connection_to_alice = await Connection.deserialize(connection_data)
        await connection_to_alice.update_state()
        connection_state = await connection_to_alice.get_state()
        if connection_state == State.Accepted:
            break
        else:
            connection_data = await connection_to_alice.serialize()
            connection_to_alice.release()
            connection_to_alice = None
            sleep(5)

    print("Serialize connection")
    connection_data = await connection_to_alice.serialize()
    connection_to_alice.release()
    connection_to_alice = None

    option = input('(1) Issue Credential, (2) Send Proof Request, (X) Exit? [1/2/X] ')
    while option != 'X' and option != 'x':
        print("Deserialize connection")
        my_connection = await Connection.deserialize(connection_data)
        sleep(2)

        if option == '1':
            await send_credential_request(my_connection, cred_def_handle)
        elif option == '2':
            await send_proof_request(my_connection, config['institution_did'])

        sleep(2)
        print("Serialize connection")
        connection_data = await my_connection.serialize()
        my_connection.release()
        my_connection = None

        option = input('(1) Issue Credential, (2) Send Proof Request, (X) Exit? [1/2/X] ')

    print("Done, pause before exiting program")
    sleep(2)


async def send_credential_request(my_connection, cred_def_handle):
    schema_attrs = {
        'name': 'alice',
        'date': '05-2018',
        'degree': 'maths',
    }

    print("#12 Create an IssuerCredential object using the schema and credential definition")
    credential = await IssuerCredential.create('alice_degree', schema_attrs, cred_def_handle, 'cred', '0')

    print("#13 Issue credential offer to alice")
    await credential.send_offer(my_connection)

    # serialize/deserialize credential - waiting for Alice to rspond with Credential Request
    credential_data = await credential.serialize()

    while True:
        print("#14 Poll agency and wait for alice to send a credential request")
        my_credential = await IssuerCredential.deserialize(credential_data)
        await my_credential.update_state()
        credential_state = await my_credential.get_state()
        if credential_state == State.RequestReceived:
            break
        else:
            credential_data = await my_credential.serialize()
            sleep(2)

    print("#17 Issue credential to alice")
    await my_credential.send_credential(my_connection)

    # serialize/deserialize credential - waiting for Alice to accept credential
    credential_data = await my_credential.serialize()

    while True:
        print("#18 Wait for alice to accept credential")
        my_credential2 = await IssuerCredential.deserialize(credential_data)
        await my_credential2.update_state()
        credential_state = await my_credential2.get_state()
        if credential_state == State.Accepted:
            break
        else:
            credential_data = await my_credential2.serialize()
            sleep(2)

    print("Done")


async def send_proof_request(my_connection, institution_did):
    proof_attrs = [
        {'name': 'name', 'restrictions': [{'issuer_did': institution_did}]},
        {'name': 'date', 'restrictions': [{'issuer_did': institution_did}]},
        {'name': 'degree', 'restrictions': [{'issuer_did': institution_did}]}
    ]

    print("#19 Create a Proof object")
    proof = await Proof.create('proof_uuid', 'proof_from_alice', proof_attrs, {})

    print("#20 Request proof of degree from alice")
    await proof.request_proof(my_connection)

    # serialize/deserialize proof
    proof_data = await proof.serialize()

    while True:
        print("#21 Poll agency and wait for alice to provide proof")
        my_proof = await Proof.deserialize(proof_data)
        await my_proof.update_state()
        proof_state = await my_proof.get_state()
        if proof_state == State.Accepted:
            break
        else:
            proof_data = await my_proof.serialize()
            sleep(2)

    print("#27 Process the proof provided by alice")
    await my_proof.get_proof(my_connection)

    print("#28 Check if proof is valid")
    if my_proof.proof_state == ProofState.Verified:
        print("proof is verified!!")
    else:
        print("could not verify proof :(")

    print("Done")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
