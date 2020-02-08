import sys
import asyncio
import json
import random
from ctypes import cdll
from time import sleep

from vcx.api.connection import Connection
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

from demo_utils import *

# logging.basicConfig(level=logging.DEBUG) uncomment to get logs


# 'agency_url': URL of the agency
# 'agency_did':  public DID of the agency
# 'agency_verkey': public verkey of the agency
# 'wallet_name': name for newly created encrypted wallet
# 'wallet_key': encryption key for encoding wallet
# 'payment_method': method that will be used for payments
provisionConfig = {
    'agency_url': 'http://localhost:8080',
    'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
    'agency_verkey': 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
    'wallet_name': 'faber_wallet_' + str(random.randint(100, 999)),
    'wallet_key': '123',
    'payment_method':  'null',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '2.0',
    'communication_method': 'aries'
}


if len(sys.argv) > 1 and sys.argv[1] == '--postgres':
    # load postgres dll and configure postgres wallet
    load_postgres_plugin(provisionConfig)

    print("Success, loaded postgres wallet storage")


async def main():
    if len(sys.argv) > 1 and sys.argv[1] == '--postgres':
        # create wallet in advance
        await create_postgres_wallet(provisionConfig)

    payment_plugin = cdll.LoadLibrary("/usr/local/lib/libnullpay" + file_ext())
    payment_plugin.nullpay_init()

    handled_offers = []
    handled_requests = []

    print("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to faber
    config['institution_name'] = 'Faber'
    config['institution_logo_url'] = 'http://robohash.org/234'
    config['genesis_path'] = 'docker.txn'
    
    print("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))

    print("#3 Create a new schema and cred def on the ledger")
    schema_uuid = 'schema_uuid'
    schema_name = 'degree schema'
    schema_attrs = ['name', 'date', 'degree', 'age']
    creddef_uuid = 'credef_uuid'
    creddef_name = 'degree'
    cred_def_json = await create_schema_and_cred_def(schema_uuid, schema_name, schema_attrs, creddef_uuid, creddef_name)

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

    option = input('(1) Issue Credential, (2) Send Proof Request, (3) Poll for Messages (X) Exit? [1/2/3/X] ')
    while option != 'X' and option != 'x':
        print("Deserialize connection")
        my_connection = await Connection.deserialize(connection_data)
        sleep(2)

        if option == '1':
            schema_attrs = {
                'name': 'alice',
                'date': '05-2018',
                'degree': 'maths',
                'age': '24',
            }
            cred_tag = 'alice_degree'
            cred_name = 'cred'

            await send_credential_request(my_connection, cred_def_json, schema_attrs, cred_tag, cred_name)

        elif option == '2':
            proof_attrs = [
                {'name': 'name', 'restrictions': [{'issuer_did': config['institution_did']}]},
                {'name': 'date', 'restrictions': [{'issuer_did': config['institution_did']}]},
                {'name': 'degree', 'restrictions': [{'issuer_did': config['institution_did']}]},
                {'name': 'self_attested_thing'}
            ]
            proof_predicates = [{'name':'age', 'p_type':'>=', 'p_value':18},]
            proof_uuid = 'proof_uuid'
            proof_name = 'proof_from_alice'

            await send_proof_request(my_connection, config['institution_did'], proof_attrs, proof_uuid, proof_name, proof_predicates)

        elif option == '3':
            await handle_messages(my_connection, handled_offers, handled_requests)

        sleep(2)
        print("Serialize connection")
        connection_data = await my_connection.serialize()
        my_connection.release()
        my_connection = None

        option = input('(1) Issue Credential, (2) Send Proof Request, (3) Poll for Messages (X) Exit? [1/2/3/X] ')

    print("Done, pause before exiting program")
    sleep(2)


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
