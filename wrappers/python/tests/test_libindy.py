from indy.libindy import LibIndy

import logging

logging.basicConfig(level=logging.DEBUG)


def test_libindy_loading_works():
    assert LibIndy.cdll() is not None
