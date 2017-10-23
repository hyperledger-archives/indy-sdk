import json
import base64

from indy import agent, signus

import pytest

message = '{"reqId":1496822211362017764}'.encode('utf-8')


@pytest.mark.asyncio
async def check_message(wallet_handle, recipient_did, encrypted_msg):
    decrypted_message = await signus.decrypt_sealed(wallet_handle, recipient_did, encrypted_msg)
    decrypted_msg = json.loads(decrypted_message.decode("utf-8"))

    assert not decrypted_msg['auth']
    assert message == base64.b64decode(decrypted_msg['msg'])


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_key(wallet_handle, identity_steward1):
    (recipient_did, recipient_verkey) = identity_steward1
    encrypted_msg = await agent.prep_anonymous_msg(recipient_verkey, message)
    check_message(wallet_handle, recipient_did, encrypted_msg)
