import json

from indy import payment

import pytest


@pytest.mark.asyncio
async def test_list_payment_addresses_works(wallet_handle):
    addresses = json.loads(await payment.list_payment_addresses(wallet_handle))
    assert 0 == len(addresses)
