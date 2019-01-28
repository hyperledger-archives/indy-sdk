import asyncio
import json
import logging
from os import environ
from pathlib import Path
from shutil import rmtree
from tempfile import gettempdir

import pytest

from indy import wallet, pool, did, ledger

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(scope="session")
def event_loop():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(pool.set_protocol_version(2))
    yield loop
    loop.close()


@pytest.fixture
def seed_trustee1():
    return "000000000000000000000000Trustee1"


@pytest.fixture
def seed_steward1():
    return "000000000000000000000000Steward1"


@pytest.fixture
def seed_my1():
    return "00000000000000000000000000000My1"


@pytest.fixture
def seed_my2():
    return "00000000000000000000000000000My2"


@pytest.fixture
def did_my():
    return "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"


@pytest.fixture
def did_my1():
    return "VsKV7grR1BUE29mG2Fm2kX"


@pytest.fixture
def did_my2():
    return "2PRyVHmkXQnQzJQKxHxnXC"


@pytest.fixture
def did_trustee():
    return "V4SGRU86Z58d6TV7PBUe6f"


@pytest.fixture
def verkey_my1():
    return "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"


@pytest.fixture
def verkey_my2():
    return "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn"


@pytest.fixture
def message():
    return '{"reqId":1496822211362017764}'.encode('utf-8')

@pytest.fixture
def pack_message():
    return '{"reqId":1496822211362017764}'


@pytest.fixture
def endpoint():
    return "127.0.0.1:9700"


@pytest.fixture
def crypto_type():
    return "ed25519"


@pytest.fixture
def metadata():
    return "ed25519"


@pytest.fixture
def path_temp():
    logger = logging.getLogger(__name__)
    logger.debug("path_temp: >>>")

    path = Path(gettempdir()).joinpath("indy_client")

    if path.exists():
        logger.debug("path_temp: Cleanup tmp path: %s", path)
        rmtree(str(path))

    logger.debug("path_temp: yield: %r", path)
    yield path

    if path.exists():
        logger.debug("path_temp: Cleanup tmp path: %s", path)
        rmtree(str(path))

    logger.debug("path_temp: <<<")


@pytest.fixture
def path_home() -> Path:
    logger = logging.getLogger(__name__)
    logger.debug("path_home: >>>")

    path = Path.home().joinpath(".indy_client")

    if path.exists():
        logger.debug("path_home: Cleanup home path: %r", path)
        rmtree(str(path))

    logger.debug("path_home: yield: %r", path)
    yield path

    if path.exists():
        logger.debug("path_home: Cleanup home path: %r", path)
        rmtree(str(path))

    logger.debug("path_home: <<<")


@pytest.fixture
def wallet_type():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_type: >>>")

    res = "default"

    logger.debug("wallet_type: <<< res: %r", res)
    return res


@pytest.fixture
def credentials():
    logger = logging.getLogger(__name__)
    logger.debug("credentials: >>>")

    res = '{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method": "RAW"}'

    logger.debug("credentials: <<< res: %r", res)
    return res


@pytest.fixture
def wallet_config():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_config: >>>")

    res = '{"id":"wallet1"}'

    logger.debug("wallet_config: <<< res: %r", res)
    return res


@pytest.fixture
def export_key():
    return "export_key"


@pytest.fixture
def export_path(path_temp):
    return str(path_temp.joinpath("export_file"))


@pytest.fixture
def export_config(export_path, export_key):
    return json.dumps({
        'path': export_path,
        'key': export_key
    })


@pytest.fixture
def xwallet_cleanup():
    logger = logging.getLogger(__name__)
    logger.debug("wallet_cleanup: >>>")

    res = True

    logger.debug("wallet_cleanup: <<< res: %r", res)
    return res


# noinspection PyUnusedLocal
@pytest.fixture
def xwallet(event_loop, xwallet_cleanup, path_home, wallet_config, credentials):
    logger = logging.getLogger(__name__)
    logger.debug("xwallet: >>> xwallet_cleanup: %r, path_home: %r, wallet_config: %r, credentials: %r",
                 xwallet_cleanup,
                 path_home,
                 wallet_config,
                 credentials)

    logger.debug("xwallet: Creating wallet")
    event_loop.run_until_complete(wallet.create_wallet(wallet_config, credentials))

    logger.debug("xwallet: yield")
    yield

    logger.debug("xwallet: Deleting wallet")
    event_loop.run_until_complete(wallet.delete_wallet(wallet_config, credentials)) if xwallet_cleanup else None

    logger.debug("xwallet: <<<")


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
def wallet_handle(event_loop, xwallet, wallet_config, credentials, wallet_handle_cleanup):
    logger = logging.getLogger(__name__)
    logger.debug(
        "wallet_handle: >>> xwallet: %r, wallet_config: %r, credentials: %r, wallet_handle_cleanup: %r",
        xwallet,
        wallet_config,
        credentials,
        wallet_handle_cleanup)

    logger.debug("wallet_handle: Opening wallet")
    wallet_handle = event_loop.run_until_complete(wallet.open_wallet(wallet_config, credentials))
    assert type(wallet_handle) is int

    logger.debug("wallet_handle: yield %r", wallet_handle)
    yield wallet_handle

    logger.debug("wallet_handle: Closing wallet")
    event_loop.run_until_complete(wallet.close_wallet(wallet_handle)) if wallet_handle_cleanup else None

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
                        '{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}'.format(
                            pool_ip, pool_ip),
                        '{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}'.format(
                            pool_ip, pool_ip),
                        '{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}'.format(
                            pool_ip, pool_ip),
                        '{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}'.format(
                            pool_ip, pool_ip)
                    ][0:pool_genesis_txn_count])

    logger.debug("pool_genesis_txn_data: <<< res: %r", res)
    return res


@pytest.fixture
def pool_genesis_txn_path(pool_name, path_temp):
    logger = logging.getLogger(__name__)
    logger.debug("pool_genesis_txn_path: >>> pool_name: %r",
                 pool_name)

    res = path_temp.joinpath("{}.txn".format(pool_name))

    logger.debug("pool_genesis_txn_path: <<< res: %r", res)
    return res


# noinspection PyUnusedLocal
@pytest.fixture
def pool_genesis_txn_file(pool_genesis_txn_path, pool_genesis_txn_data):
    logger = logging.getLogger(__name__)
    logger.debug("pool_genesis_txn_file: >>> pool_genesis_txn_path: %r, pool_genesis_txn_data: %r",
                 pool_genesis_txn_path,
                 pool_genesis_txn_data)

    pool_genesis_txn_path.parent.mkdir(parents=True, exist_ok=True)

    with open(str(pool_genesis_txn_path), "w+") as f:
        f.writelines(pool_genesis_txn_data)

    logger.debug("pool_genesis_txn_file: <<<")


@pytest.fixture
def pool_ledger_config_cleanup():
    return True


# noinspection PyUnusedLocal
@pytest.fixture
def pool_ledger_config(event_loop, pool_name, pool_genesis_txn_path, pool_genesis_txn_file,
                       pool_ledger_config_cleanup, path_home):
    logger = logging.getLogger(__name__)
    logger.debug("pool_ledger_config: >>> pool_name: %r, pool_genesis_txn_path: %r, pool_genesis_txn_file: %r,"
                 " pool_ledger_config_cleanup: %r, path_home: %r",
                 pool_name,
                 pool_genesis_txn_path,
                 pool_genesis_txn_file,
                 pool_ledger_config_cleanup,
                 path_home)

    logger.debug("pool_ledger_config: Creating pool ledger config")
    event_loop.run_until_complete(pool.create_pool_ledger_config(
        pool_name,
        json.dumps({
            "genesis_txn": str(pool_genesis_txn_path)
        })))

    logger.debug("pool_ledger_config: yield")
    yield

    logger.debug("pool_ledger_config: Deleting pool ledger config")
    event_loop.run_until_complete(pool.delete_pool_ledger_config(pool_name)) if pool_ledger_config_cleanup else None

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


@pytest.fixture
def protocol_version():
    logger = logging.getLogger(__name__)
    logger.debug("protocol_version: >>>")

    res = 2

    logger.debug("protocol_version: <<< res: %r", res)
    return res


# noinspection PyUnusedLocal
@pytest.fixture
def pool_handle(event_loop, pool_name, pool_ledger_config, pool_config, pool_handle_cleanup, protocol_version):
    logger = logging.getLogger(__name__)
    logger.debug("pool_handle: >>> pool_name: %r, pool_ledger_config: %r, pool_config: %r, pool_handle_cleanup: %r,"
                 " protocol_version: %r",
                 pool_name,
                 pool_ledger_config,
                 pool_config,
                 pool_handle_cleanup,
                 protocol_version)

    logger.debug("pool_handle: Opening pool ledger")
    pool_handle = event_loop.run_until_complete(pool.open_pool_ledger(pool_name, pool_config))
    assert type(pool_handle) is int

    logger.debug("pool_handle: yield: %r", pool_handle)
    yield pool_handle

    logger.debug("pool_handle: Closing pool ledger")
    event_loop.run_until_complete(pool.close_pool_ledger(pool_handle)) if pool_handle_cleanup else None

    logger.debug("pool_handle: <<<")


@pytest.fixture
async def identity_trustee1(wallet_handle, seed_trustee1):
    (trustee_did, trustee_verkey) = await did.create_and_store_my_did(wallet_handle,
                                                                      json.dumps({"seed": seed_trustee1}))
    return trustee_did, trustee_verkey


@pytest.fixture
async def identity_steward1(wallet_handle, seed_steward1):
    (steward_did, steward_verkey) = await did.create_and_store_my_did(wallet_handle,
                                                                      json.dumps({"seed": seed_steward1}))
    return steward_did, steward_verkey


@pytest.fixture
async def identity_my1(wallet_handle, pool_handle, seed_my1, ):
    (my_did, my_verkey) = await did.create_and_store_my_did(wallet_handle,
                                                            json.dumps({"seed": seed_my1, 'cid': True}))

    return my_did, my_verkey


@pytest.fixture
async def identity_my(wallet_handle, pool_handle, identity_trustee1, seed_my1, ):
    (trustee_did, trustee_verkey) = identity_trustee1

    (my_did, my_verkey) = await did.create_and_store_my_did(wallet_handle, "{}")

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, 'TRUSTEE')
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    return my_did, my_verkey


@pytest.fixture
async def identity_my2(wallet_handle, identity_trustee1, seed_my2, ):
    (trustee_did, trustee_verkey) = identity_trustee1

    (my_did, my_verkey) = await did.create_and_store_my_did(wallet_handle, json.dumps({"seed": seed_my2}))

    await did.store_their_did(wallet_handle, json.dumps({'did': trustee_did, 'verkey': trustee_verkey}))
    return my_did, my_verkey


@pytest.fixture
async def key_my1(wallet_handle, seed_my1, ):
    key = await did.create_key(wallet_handle, json.dumps({"seed": seed_my1}))
    return key
