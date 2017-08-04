import json
import logging
from distutils import dirname
from os import environ, makedirs

import pytest

from indy import wallet, pool
from tests.utils import storage as storage_utils

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture
def trustee1_seed():
    logger = logging.getLogger(__name__)
    logger.debug("trustee1_seed: >>>")

    res = "000000000000000000000000Trustee1"

    logger.debug("trustee1_seed: <<< res: %r", res)
    return res


@pytest.fixture
def cleanup_storage():
    logger = logging.getLogger(__name__)
    logger.debug("cleanup_storage: >>>")

    storage_utils.cleanup()

    logger.debug("cleanup_storage: yield")
    yield

    storage_utils.cleanup()
    logger.debug("cleanup_storage: <<<")


@pytest.fixture
def wallet_name():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_name: >>>")

    res = "wallet1"

    logger.debug("wallet_name: <<< res: %r", res)
    return res


@pytest.fixture
def wallet_type():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_type: >>>")

    res = "default"

    logger.debug("wallet_type: <<< res: %r", res)
    return res


@pytest.fixture
def wallet_config_cleanup():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_cleanup: >>>")

    res = True

    logger.debug("wallet_cleanup: <<< res: %r", res)
    return res


# noinspection PyUnusedLocal
@pytest.fixture
async def wallet_config(pool_name, wallet_name, wallet_type, wallet_config_cleanup, cleanup_storage):
    logger = logging.getLogger(__name__)
    logger.debug("wallet_config: >>> pool_name: %r, wallet_type: %r, wallet_config_cleanup: %r, cleanup_storage: %r",
                 pool_name,
                 wallet_type,
                 wallet_config,
                 cleanup_storage)

    logger.debug("wallet_config: Creating wallet")
    await wallet.create_wallet(pool_name, wallet_name, wallet_type, None, None)

    logger.debug("wallet_config: yield")
    yield

    logger.debug("wallet_config: Deleting wallet")
    await wallet.delete_wallet(wallet_name, None) if wallet_config_cleanup else None

    logger.debug("wallet_config: <<<")


@pytest.fixture
def wallet_runtime_config():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_runtime_config: >>>")

    res = None

    logger.debug("wallet_runtime_config: <<< res: %r", res)
    return res


@pytest.fixture
def wallet_handle_cleanup():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_handle_cleanup: >>>")

    res = True

    logger.debug("wallet_handle_cleanup: <<< res: %r", res)
    return res


@pytest.fixture
async def wallet_handle(wallet_name, wallet_config, wallet_runtime_config, wallet_handle_cleanup):
    logger = logging.getLogger(__name__)
    logger.debug(
        "wallet_handle: >>> wallet_name: %r, wallet_config: %r, wallet_runtime_config: %r, wallet_handle_cleanup: %r",
        wallet_name,
        wallet_config,
        wallet_runtime_config,
        wallet_handle_cleanup)

    logger.debug("wallet_handle: Opening wallet")
    wallet_handle = await wallet.open_wallet(wallet_name, wallet_runtime_config, None)
    assert type(wallet_handle) is int

    logger.debug("wallet_handle: yield %r", wallet_handle)
    yield wallet_handle

    logger.debug("wallet_handle: Closing wallet")
    await wallet.close_wallet(wallet_handle) if wallet_handle_cleanup else None

    logger.debug("wallet_handle: <<<")


@pytest.fixture
def pool_name():
    logger = logging.getLogger(__name__)
    logger.debug("pool_name: >>>")

    res = "pool1"

    logger.debug("pool_name: <<< res: %r", res)
    return res


@pytest.fixture
def pool_ip():
    logger = logging.getLogger(__name__)
    logger.debug("pool_ip: >>>")

    res = environ.get("TEST_POOL_IP", "127.0.0.1")

    logger.debug("pool_ip: <<< res: %r", res)
    return res


@pytest.fixture
def pool_genesis_txn_count():
    logger = logging.getLogger(__name__)
    logger.debug("pool_genesis_txn_count: >>>")

    res = 4

    logger.debug("pool_genesis_txn_count: <<< res: %r", res)
    return res


@pytest.fixture
def pool_genesis_txn_data(pool_genesis_txn_count, pool_ip):
    logger = logging.getLogger(__name__)
    logger.debug("pool_genesis_txn_data: >>> pool_genesis_txn_count: %r, pool_ip: %r",
                 pool_genesis_txn_count,
                 pool_ip)

    assert 0 < pool_genesis_txn_count <= 4

    res = "\n".join([
                        '{{"data":{{"alias":"Node1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}}'.format(
                            pool_ip, pool_ip),
                        '{{"data":{{"alias":"Node2","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}}'.format(
                            pool_ip, pool_ip),
                        '{{"data":{{"alias":"Node3","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}}'.format(
                            pool_ip, pool_ip),
                        '{{"data":{{"alias":"Node4","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}}'.format(
                            pool_ip, pool_ip)
                    ][0:pool_genesis_txn_count])

    logger.debug("pool_genesis_txn_data: <<< res: %r", res)
    return res


@pytest.fixture
def pool_genesis_txn_path(pool_name):
    logger = logging.getLogger(__name__)
    logger.debug("pool_genesis_txn_path: >>> pool_name: %r",
                 pool_name)

    res = storage_utils.indy_temp_path().joinpath("{}.txn".format(pool_name))

    logger.debug("pool_genesis_txn_path: <<< res: %r", res)
    return res


# noinspection PyUnusedLocal
@pytest.fixture
def pool_genesis_txn_file(pool_genesis_txn_path, pool_genesis_txn_data, cleanup_storage):
    logger = logging.getLogger(__name__)
    logger.debug("pool_genesis_txn_file: >>> pool_genesis_txn_path: %r, pool_genesis_txn_data: %r, cleanup_storage: %r",
                 pool_genesis_txn_path,
                 pool_genesis_txn_data,
                 cleanup_storage)

    makedirs(dirname(pool_genesis_txn_path))

    with open(str(pool_genesis_txn_path), "w+") as f:
        f.writelines(pool_genesis_txn_data)

    logger.debug("pool_genesis_txn_file: <<<")


@pytest.fixture
def pool_ledger_config_cleanup():
    return True


# noinspection PyUnusedLocal
@pytest.fixture
async def pool_ledger_config(pool_name, pool_genesis_txn_path, pool_genesis_txn_file, pool_ledger_config_cleanup):
    logger = logging.getLogger(__name__)
    logger.debug("pool_ledger_config: >>> pool_name: %r, pool_genesis_txn_path: %r, pool_genesis_txn_file: %r,"
                 " pool_ledger_config_cleanup: %r",
                 pool_name,
                 pool_genesis_txn_path,
                 pool_genesis_txn_file,
                 pool_ledger_config_cleanup)

    logger.debug("pool_ledger_config: Creating pool ledger config")
    await pool.create_pool_ledger_config(
        pool_name,
        json.dumps({
            "genesis_txn": str(pool_genesis_txn_path)
        }))

    logger.debug("pool_ledger_config: yield")
    yield

    logger.debug("pool_ledger_config: Deleting pool ledger config")
    await pool.delete_pool_ledger_config(pool_name) if pool_ledger_config_cleanup else None

    logger.debug("pool_ledger_config: <<<")


@pytest.fixture
def pool_handle_cleanup():
    logger = logging.getLogger(__name__)
    logger.debug("pool_handle_cleanup: >>>")

    res = True

    logger.debug("pool_handle_cleanup: <<< res: %r", res)
    return res


@pytest.fixture
def pool_config():
    logger = logging.getLogger(__name__)
    logger.debug("pool_config: >>>")

    res = None

    logger.debug("pool_config: <<< res: %r", res)
    return res


# noinspection PyUnusedLocal
@pytest.fixture
async def pool_handle(pool_name, pool_ledger_config, pool_config, pool_handle_cleanup):
    logger = logging.getLogger(__name__)
    logger.debug("pool_handle: >>> pool_name: %r, pool_ledger_config: %r, pool_config: %r, pool_handle_cleanup: %r",
                 pool_name,
                 pool_ledger_config,
                 pool_config,
                 pool_handle_cleanup)

    logger.debug("pool_handle: Opening pool ledger")
    pool_handle = await pool.open_pool_ledger(pool_name, pool_config)
    assert type(pool_handle) is int

    logger.debug("pool_handle: yield: %r", pool_handle)
    yield pool_handle

    logger.debug("pool_handle: Closing pool ledger")
    await pool.close_pool_ledger(pool_handle) if pool_handle_cleanup else None

    logger.debug("pool_handle: <<<")
