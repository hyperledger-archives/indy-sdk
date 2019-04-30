import json
import pytest

from indy import non_secrets


@pytest.mark.asyncio
async def test_purge_schema_cache_works(wallet_handle):

    options_json = {
        "maxAge": -1,
    }

    await non_secrets.purge_schema_cache(wallet_handle, json.dumps(options_json))
