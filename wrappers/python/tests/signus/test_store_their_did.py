from indy import signus

import pytest


@pytest.mark.asyncio
async def test_store_their_did_works(wallet_handle):
    await signus.store_their_did(wallet_handle, '{"did":"8wZcEriaNLNKtteJvx7f8i"}')
