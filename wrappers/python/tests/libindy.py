from indy import libindy


def test_libindy_loading_works():
    assert libindy.cdll is not None
