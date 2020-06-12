import asyncio
import json
from ctypes import cdll
from time import sleep

from demo_utils import file_ext
from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State
from vc_auth_oidc.alice_vc_auth import handle_challenge


# logging.basicConfig(level=logging.DEBUG) uncomment to get logs

provisionConfig = {
    'agency_url': 'http://localhost:8080',
    'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
    'agency_verkey': 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
    'wallet_name': 'alternate_wallet',
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '3.0',
    'storage_config': '{"path":"/tmp/alternate_path/"}'
}


async def main():
    payment_plugin = cdll.LoadLibrary('libnullpay' + file_ext())
    payment_plugin.nullpay_init()

    print("Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to alice
    config['institution_name'] = 'alice'
    config['institution_logo_url'] = 'http://robohash.org/456'
    config['genesis_path'] = 'docker.txn'
    config['payment_method'] = 'null'

    config = json.dumps(config)

    print('Alice config:\n  ' + config)

    print("Initialize libvcx with new configuration")
    await vcx_init_with_config(config)

    print("Initialized alternate wallet path")

    connection_to_faber = None


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
