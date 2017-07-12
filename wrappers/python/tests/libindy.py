import logging

logging.basicConfig(level=logging.DEBUG)

def test_libindy_loading_works():
    import indy.libindy as libindy
    assert libindy.cdll != None