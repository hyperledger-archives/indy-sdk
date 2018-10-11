import pytest

from indy import libindy


# noinspection PyUnusedLocal
@pytest.mark.sync
def test_set_runtime_config():
    libindy.set_runtime_config('{"crypto_thread_pool_size": 2}')
