import asyncio
import json
import random
import string
from ctypes import cdll
import platform

from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config


EXTENSION = {"darwin": ".dylib", "linux": ".so", "win32": ".dll", 'windows': '.dll'}


def file_ext():
    your_platform = platform.system().lower()
    return EXTENSION[your_platform] if (your_platform in EXTENSION) else '.so'


def load_payment_plugin():
    payment_plugin = cdll.LoadLibrary('libnullpay' + file_ext())
    payment_plugin.nullpay_init()


def rand_string() -> str:
    return ''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(20))


async def init_vcx():
    load_payment_plugin()

    config = {
        'agency_url': 'http://localhost:8080',
        'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
        'agency_verkey': 'Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR',
        'wallet_name': rand_string(),
        'wallet_key': '123',
        'payment_method': 'null',
        'enterprise_seed': '000000000000000000000000Trustee1',
        'protocol_type': '2.0',
        'communication_method': 'aries'
    }

    print("Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(config))
    config = json.loads(config)
    # Set some additional configuration options specific to faber
    config['pool_name'] = rand_string()
    config['institution_name'] = rand_string()
    config['institution_logo_url'] = 'http://robohash.org/234'
    config['genesis_path'] = 'docker.txn'

    print("Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))


def run_coroutine(coroutine, loop=None):
    if loop is None:
        loop = asyncio.get_event_loop()
    return loop.run_until_complete(coroutine())


def run_coroutine_in_new_loop(coroutine):
    loop = asyncio.new_event_loop()
    return loop.run_until_complete(coroutine())
