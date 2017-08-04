from indy import agent
from indy import ledger, signus, wallet, pool
from indy.pool import open_pool_ledger

import pytest
import json

from tests.utils.pool import create_genesis_txn_file_for_test_pool


@pytest.mark.asyncio
async def test_agent_demo_works(cleanup_storage):
    # 1. Create ledger config from genesis txn file
    path = create_genesis_txn_file_for_test_pool('pool_1')
    pool_config = json.dumps({"genesis_txn": str(path)})
    await pool.create_pool_ledger_config('pool_1', pool_config)

    # 2. Open pool ledger
    pool_handle = await open_pool_ledger('pool_1', None)

    # 3. Create and Open Listener Wallet. Gets wallet handle
    await wallet.create_wallet('pool_1', 'listener_wallet', None, None, None)
    listener_wallet_handle = await wallet.open_wallet('listener_wallet', None, None)

    # 4. Create and Open Sender Wallet. Gets wallet handle
    await wallet.create_wallet('pool_1', 'sender_wallet', None, None, None)
    sender_wallet_handle = await wallet.open_wallet('sender_wallet', None, None)

    # 5. Create Listener DID
    (listener_did, listener_verkey, listener_pk) = await signus.create_and_store_my_did(listener_wallet_handle, "{}")

    # 6. Create Sender DID from Trustee1 seed
    (sender_did, sender_verkey, sender_pk) = \
        await signus.create_and_store_my_did(sender_wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')

    # 7. Prepare and send NYM transaction
    nym_txn_req = await ledger.build_nym_request(sender_did, listener_did, listener_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, sender_wallet_handle, sender_did, nym_txn_req)

    # 8. Prepare and send Attrib request
    raw = json.dumps({
        "endpoint": {
            "ha": "127.0.0.1:5555",
            "verkey": listener_pk
        }
    })

    attrib_txn_req = await ledger.build_attrib_request(listener_did, listener_did, None, raw, None)
    await ledger.sign_and_submit_request(pool_handle, listener_wallet_handle, listener_did, attrib_txn_req)

    # 9. Start listener on endpoint
    listener_handle = await agent.agent_listen("127.0.0.1:5555")

    # 10. Allow listener accept incoming connection for specific DID (listener_did)
    await agent.agent_add_identity(listener_handle, pool_handle, listener_wallet_handle, listener_did)

    # 11. Initiate connection from sender to listener
    connection_handle = await agent.agent_connect(pool_handle, sender_wallet_handle, sender_did, listener_did)
    event = await agent.agent_wait_for_event([listener_handle])
    inc_con_handle = event.connection_handle

    # 12. Send test message from sender to listener
    message = 'msg_from_sender_to_listener'
    await agent.agent_send(connection_handle, message)

    message_event = await agent.agent_wait_for_event([listener_handle, inc_con_handle])  # type: agent.MessageEvent
    assert message_event.message == message

    # 13. Close connection, listener, wallets, pool
    await agent.agent_close_listener(listener_handle)
    await agent.agent_close_connection(connection_handle)

    await wallet.close_wallet(listener_wallet_handle)
    await wallet.close_wallet(sender_wallet_handle)
    await pool.close_pool_ledger(pool_handle)
