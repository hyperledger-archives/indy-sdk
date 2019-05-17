import json
import pytest

from indy import cache


@pytest.mark.asyncio
async def test_purge_schema_cache_works(wallet_handle):

    options_json = {
        "maxAge": -1,
    }

    await cache.purge_schema_cache(wallet_handle, json.dumps(options_json))
