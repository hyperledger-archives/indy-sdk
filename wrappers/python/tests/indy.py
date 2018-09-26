import pytest

from indy import libindy


# noinspection PyUnusedLocal
@pytest.mark.sync
def test_create_wallet_works_for_duplicate_name():
    libindy.set_crypto_thread_pool_size(2)
