from indy import agent, crypto, wallet

import logging

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)

message = '{"reqId":1496822211362017764}'.encode('utf-8')

async def demo():
    logger.info("Agent sample -> started")
    # 1. Create and open wallets for Alice and Bob
    await wallet.create_wallet("no pool", "alice_wallet", None, None, None)
    alice_wallet_handle = await wallet.open_wallet("alice_wallet", None, None)
    await wallet.create_wallet("no pool", "bob_wallet", None, None, None)
    bob_wallet_handle = await wallet.open_wallet("bob_wallet", None, None)

    # 2. Create keys for Alice and Bob
    alice_key = await crypto.create_key(alice_wallet_handle, '{}')
    bob_key = await crypto.create_key(bob_wallet_handle, '{}')

    # 3. Prepare authenticated message from Alice to Bob
    encrypted_auth_msg = await agent.prep_msg(alice_wallet_handle, alice_key, bob_key, message)

    # 4. Parse authenticated message on Bob's side
    sender_auth, decrypted_auth_msg = await agent.parse_msg(bob_wallet_handle, bob_key, encrypted_auth_msg)
    assert sender_auth == alice_key
    assert decrypted_auth_msg == message

    # 5. Prepare anonymous message from Bob to Alice
    encrypted_anon_msg = await agent.prep_anonymous_msg(alice_key, message)

    # 6. Parse anonymous message on Alice's side
    sender_anon, decrypted_anon_msg = await agent.parse_msg(alice_wallet_handle, alice_key, encrypted_anon_msg)
    assert not sender_anon
    assert decrypted_anon_msg == message

    await wallet.close_wallet(alice_wallet_handle)
    await wallet.close_wallet(bob_wallet_handle)
    await wallet.delete_wallet("alice_wallet", None)
    await wallet.delete_wallet("bob_wallet", None)

    logger.info("Agent sample -> completed")
