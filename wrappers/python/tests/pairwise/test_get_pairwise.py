from indy import IndyError
from indy import did
from indy import pairwise

import pytest
import json

from indy.error import ErrorCode

def ppjson(dumpit):
    rv = json.dumps(json.loads(dumpit) if isinstance(dumpit, str) else dumpit, indent=4)
    return rv

@pytest.mark.asyncio
async def test_get_pairwise_works_with_metadata(wallet_handle, identity_my2, identity_trustee1):
    (my_did, _) = identity_my2
    (their_did, _) = identity_trustee1
    metadata = 'some metadata'
    await pairwise.create_pairwise(wallet_handle, their_did, my_did, metadata)
    # await pairwise.create_pairwise(wallet_handle, their_did, my_did, metadata)
    # await pairwise.create_pairwise(wallet_handle, 'LjgpST2rjsoxYegQDRm7EL', my_did, metadata)

    oney = await pairwise.get_pairwise(wallet_handle, their_did)
    print('[{}] one: {}'.format(type(oney), ppjson(oney)))
    listy = await pairwise.list_pairwise(wallet_handle)
    print('[{}] all: {}'.format(type(listy), ppjson(listy)))
    # assert {'my_did': my_did, 'metadata': metadata} == json.loads(await pairwise.get_pairwise(wallet_handle, their_did))
