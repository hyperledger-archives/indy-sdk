from indy import agent
from indy import ledger, signus, wallet, pool
from indy.pool import open_pool_ledger

from tests.utils import storage

import pytest
import logging
import json

from tests.utils.pool import create_genesis_txn_file

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_agent_demo_works():
    pool_name = "pool"
    listener_wallet_name = "listener_wallet"
    sender_wallet_name = "sender_wallet"

    # 1. Create ledger config from genesis txn file
    file_name = pool_name + '.txn'
    path = create_genesis_txn_file(file_name, None)
    pool_config = json.dumps({"genesis_txn": str(path)})
    await pool.create_pool_ledger_config(pool_name, pool_config)

    # 2. Open pool ledger
    pool_handle = await open_pool_ledger(pool_name, None)

    # 3. Create and Open Listener Wallet. Gets wallet handle
    await wallet.create_wallet(pool_name, listener_wallet_name, None, None, None)
    listener_wallet_handle = await wallet.open_wallet(listener_wallet_name, None, None)

    # 4. Create and Open Sender Wallet. Gets wallet handle
    await wallet.create_wallet(pool_name, sender_wallet_name, None, None, None)
    sender_wallet_handle = await wallet.open_wallet(sender_wallet_name, None, None)

    # 5. Create Listener DID
    (listener_did, listener_verkey, listener_pk) = await signus.create_and_store_my_did(listener_wallet_handle, "{}")

    # 6. Create Sender DID from Trustee1 seed
    (sender_did, sender_verkey, sender_pk) = \
        await signus.create_and_store_my_did(sender_wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')

    # 7. Prepare and send NYM transaction
    nym_txn_req = await ledger.build_nym_request(sender_did, listener_did, listener_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, sender_wallet_handle, sender_did, nym_txn_req.decode())

    # 8. Prepare and send GET_NYM request
    endpoint = "127.0.0.1:5555"
    raw = {
        "endpoint": {
            "ha": endpoint,
            "verkey": listener_pk
        }
    }

    # 8. Prepare and send Attrib request
    attrib_txn_req = await ledger.build_attrib_request(listener_did, listener_did, None, json.dumps(raw), None)
    await ledger.sign_and_submit_request(pool_handle, listener_wallet_handle, listener_did, attrib_txn_req.decode())

    # 8. Start listener on endpoint
    listener_handle = await agent.agent_listen(endpoint)

    # 9. Allow listener accept incoming connection for specific DID (listener_did)
    await agent.agent_add_identity(listener_handle, pool_handle, listener_wallet_handle, listener_did)

    # 10. Initiate connection from sender to listener
    connection_handle = await agent.agent_connect(pool_handle, sender_wallet_handle, sender_did, listener_did)

    # 11. Send test message from sender to listener
    message = 'msg_from_sender_to_listener'
    await agent.agent_send(connection_handle, message)

    # 12. Close connection, listener, wallets, pool
    await agent.agent_close_listener(listener_handle)
    await agent.agent_close_connection(connection_handle)

    await wallet.close_wallet(listener_wallet_handle)
    await wallet.close_wallet(sender_wallet_handle)
    await pool.close_pool_ledger(pool_handle)
