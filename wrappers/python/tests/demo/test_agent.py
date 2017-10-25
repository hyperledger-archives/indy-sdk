from indy import agent, ledger, signus, pool, wallet

import pytest
import json


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_agent_demo_works(pool_name, pool_genesis_txn_path, seed_trustee1, pool_genesis_txn_file, endpoint,
                                path_home):
    # 1. Create ledger config from genesis txn file
    pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
    await pool.create_pool_ledger_config(pool_name, pool_config)

    # 2. Open pool ledger
    pool_handle = await pool.open_pool_ledger(pool_name, None)

    # 3. Create and Open Listener Wallet. Gets wallet handle
    await wallet.create_wallet(pool_name, 'listener_wallet', None, None, None)
    listener_wallet_handle = await wallet.open_wallet('listener_wallet', None, None)

    # 4. Create and Open Sender Wallet. Gets wallet handle
    await wallet.create_wallet(pool_name, 'sender_wallet', None, None, None)
    sender_wallet_handle = await wallet.open_wallet('sender_wallet', None, None)

    # 5. Create Listener DID
    (listener_did, listener_verkey) = await signus.create_and_store_my_did(listener_wallet_handle, "{}")

    # 6. Create Sender DID from Trustee1 seed
    (sender_did, sender_verkey) = await signus.create_and_store_my_did(sender_wallet_handle,
                                                                       json.dumps({"seed": seed_trustee1}))

    # 7. Prepare and send NYM transaction
    nym_txn_req = await ledger.build_nym_request(sender_did, listener_did, listener_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, sender_wallet_handle, sender_did, nym_txn_req)

    # 8. Prepare and send Attrib request
    raw = json.dumps({
        "endpoint": {
            "ha": endpoint,
            "verkey": listener_verkey
        }
    })

    attrib_txn_req = await ledger.build_attrib_request(listener_did, listener_did, None, raw, None)
    await ledger.sign_and_submit_request(pool_handle, listener_wallet_handle, listener_did, attrib_txn_req)

    # 9. Start listener on endpoint
    listener_handle = await agent.agent_listen(endpoint)

    # 10. Allow listener accept incoming connection for specific DID (listener_did)
    await agent.agent_add_identity(listener_handle, pool_handle, listener_wallet_handle, listener_did)

    # 11. Initiate connection from sender to listener
    connection_handle = await agent.agent_connect(pool_handle, sender_wallet_handle, sender_did, listener_did)
    event = await agent.agent_wait_for_event([listener_handle])  # type: agent.ConnectionEvent
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
