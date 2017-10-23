import json

from indy import agent, signus

import pytest

message = '{"reqId":1496822211362017764}'.encode('utf-8')


@pytest.mark.asyncio
async def check_message(wallet_handle, recipient_did, sender_verkey, encrypted_msg):
    decrypted_message = await signus.decrypt_sealed(wallet_handle, recipient_did, encrypted_msg)
    decrypted_msg = json.loads(decrypted_message.decode("utf-8"))

    assert decrypted_msg['auth']
    assert sender_verkey == decrypted_msg['sender']
    assert decrypted_msg['nonce']
    assert decrypted_msg['msg']


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_key(wallet_handle, identity_steward1, seed_my1):
    (_, sender_verkey, _) = \
        await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1, 'cid': True}))
    (recipient_did, recipient_verkey) = identity_steward1
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, recipient_verkey, message)
    check_message(wallet_handle, recipient_did, sender_verkey, encrypted_msg)
