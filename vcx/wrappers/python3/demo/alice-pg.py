import sys
import asyncio
import json
import random
from ctypes import cdll, CDLL
from time import sleep
import platform

import logging

from indy import wallet
from indy.error import ErrorCode, IndyError

from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision, vcx_messages_download
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

from demo_utils import *

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
    load_postgres_plugin(provisionConfig)


async def main():
    if len(sys.argv) > 1 and sys.argv[1] == '--postgres':
        # create wallet in advance
        await create_postgres_wallet(provisionConfig)

    payment_plugin = cdll.LoadLibrary("libnullpay" + file_ext())
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


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())

