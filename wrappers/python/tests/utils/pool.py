import json
from os import environ, makedirs
from pathlib import Path
from typing import Optional

from os.path import dirname

from indy import pool
from .storage import indy_temp_path


def test_pool_ip():
    return environ.get("TEST_POOL_IP", "127.0.0.1")


def create_genesis_txn_file(pool_name: str,
                            txn_file_data: str,
                            txn_file_path: Optional[Path] = None) -> Path:
    txn_file_path = txn_file_path or indy_temp_path().joinpath("{}.txn".format(pool_name))
    makedirs(dirname(txn_file_path))

    with open(str(txn_file_path), "w+") as f:
        f.writelines(txn_file_data)

    return txn_file_path


def create_genesis_txn_file_for_test_pool(pool_name: str,
                                          nodes_count: Optional[int] = 4,
                                          txn_file_path: Optional[Path] = None):
    nodes_count = nodes_count or 4
    assert 0 < nodes_count <= 4

    pool_ip = test_pool_ip()

    node_txns = [
        '{{"data":{{"alias":"Node1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv","identifier":"Th7MpTaRZVRYnPiabds81Y","txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62","type":"0"}}'.format(pool_ip, pool_ip),
        '{{"data":{{"alias":"Node2","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb","identifier":"EbP4aYNeTHL6q385GuVpRV","txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc","type":"0"}}'.format(pool_ip, pool_ip),
        '{{"data":{{"alias":"Node3","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya","identifier":"4cU41vWW82ArfxJxHkzXPG","txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4","type":"0"}}'.format(pool_ip, pool_ip),
        '{{"data":{{"alias":"Node4","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA","identifier":"TWwCRQRZ2ZHMJFn9TzLp7W","txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008","type":"0"}}'.format(pool_ip, pool_ip)
    ]

    txn_file_data = "\n".join(node_txns[0:nodes_count])

    return create_genesis_txn_file(pool_name, txn_file_data, txn_file_path)


async def create_and_open_pool_ledger(pool_name: str = "pool_1") -> int:
    pool_config = json.dumps({
        "genesis_txn": str(create_genesis_txn_file_for_test_pool(pool_name))
    })

    await pool.create_pool_ledger_config(pool_name, pool_config)

    pool_handle = await pool.open_pool_ledger(pool_name, None)
    assert pool_handle is not None

    return pool_handle


async def close_pool_ledger(pool_handle) -> None:
    await pool.close_pool_ledger(pool_handle)
