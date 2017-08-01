from .storage import indy_temp_path, create_temp_dir
from indy import pool

import json


async def create_pool_ledger_config(config_name, nodes=None, pool_config=None, gen_txn_file_name=None):
    file_name = gen_txn_file_name or (config_name + '.txn')
    path = create_genesis_txn_file(file_name, nodes)

    pool_config = json.dumps(pool_config or {"genesis_txn": str(path)})
    await pool.create_pool_ledger_config(config_name, pool_config)


def create_genesis_txn_file(file_name, predefined_data=None):
    path = indy_temp_path().joinpath(file_name)

    default_txn = [
        "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"10.0.0.2\",\"client_port\":9702,\"node_ip\":\"10.0.0.2\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}\n",
        "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"10.0.0.2\",\"client_port\":9704,\"node_ip\":\"10.0.0.2\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"EbP4aYNeTHL6q385GuVpRV\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}\n",
        "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"10.0.0.2\",\"client_port\":9706,\"node_ip\":\"10.0.0.2\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"4cU41vWW82ArfxJxHkzXPG\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}\n",
        "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"10.0.0.2\",\"client_port\":9708,\"node_ip\":\"10.0.0.2\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"TWwCRQRZ2ZHMJFn9TzLp7W\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}\n"
    ]

    create_temp_dir()

    with open(str(path), "w+") as f:
        f.writelines(predefined_data or default_txn)

    return path


def create_default_pool_config(pool_name):
    file_name = pool_name + '.txn'
    path = indy_temp_path().joinpath(file_name)

    return {"genesis_txn": str(path)}


async def create_and_open_pool_ledger(name="pool_1"):
    await create_pool_ledger_config(name)
    pool_handle = await pool.open_pool_ledger(name, "")
    assert pool_handle is not None
    return pool_handle


async def close_pool_ledger(pool_handle):
    await pool.close_pool_ledger(pool_handle)
