import logging

import pytest

from indy import wallet
from ..utils import storage

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def cleanup_storage():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_agent_connect_works_for_all_data_in_wallet_present():
    pass
