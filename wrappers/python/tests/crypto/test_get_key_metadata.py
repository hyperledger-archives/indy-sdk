from indy import IndyError
from indy import did
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_get_key_metadata_works(wallet_handle, metadata):
    verkey = await did.create_key(wallet_handle, "{}")
    print('VERKEY: {}, {}'.format(type(verkey), verkey))
    await did.set_key_metadata(wallet_handle, verkey, metadata)
    received_metadata = await did.get_key_metadata(wallet_handle, verkey)
    print('META: {}'.format(received_metadata))
    assert metadata == received_metadata
