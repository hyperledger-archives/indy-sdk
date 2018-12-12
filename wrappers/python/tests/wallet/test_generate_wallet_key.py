import json

import pytest

from indy import wallet


@pytest.mark.asyncio
async def test_generate_wallet_key_works(wallet_config):
    key = await wallet.generate_wallet_key(None)

    credentials = {'key': key, 'key_derivation_method': 'RAW'}
    await wallet.create_wallet(wallet_config, json.dumps(credentials))
    await wallet.delete_wallet(wallet_config, json.dumps(credentials))


@pytest.mark.asyncio
async def test_generate_wallet_key_works_for_seed(wallet_config, seed_my1):
    key = await wallet.generate_wallet_key(json.dumps({'seed': seed_my1}))
    assert key == 'CwMHrEQJnwvuE8q9zbR49jyYtVxVBHNTjCPEPk1aV3cP'

    credentials = {'key': key, 'key_derivation_method': 'RAW'}
    await wallet.create_wallet(wallet_config, json.dumps(credentials))
    await wallet.delete_wallet(wallet_config, json.dumps(credentials))
