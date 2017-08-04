import pytest

from tests.utils import storage as storage_utils, wallet as wallet_utils, anoncreds as anoncreds_utils

WALLET = {
    "opened": False,
    "handle": None,
    "claim_def": None
}


# noinspection PyUnusedLocal
@pytest.fixture
async def init_common_wallet():
    global WALLET

    if WALLET["opened"]:
        yield (WALLET["handle"], WALLET["claim_def"])
        return

    storage_utils.cleanup()
    wallet_handle = await wallet_utils.create_and_open_wallet(pool_name="anoncreds_common_pool",
                                                              wallet_name="anoncreds_common_wallet")
    assert type(wallet_handle) is int
    claim_def = await anoncreds_utils.prepare_common_wallet(wallet_handle)

    WALLET = {
        "opened": True,
        "handle": wallet_handle,
        "claim_def": claim_def
    }

    yield (wallet_handle, claim_def)
