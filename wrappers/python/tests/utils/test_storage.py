from .storage import indy_home_path, indy_temp_path, cleanup


def test_storage_utils_indy_home_path_works():
    home_path = indy_home_path()
    assert '.indy' in str(home_path)


def test_storage_utils_indy_temp_path_works():
    tmp_path = indy_temp_path()
    assert 'indy' in str(tmp_path)


def test_storage_utils_cleanup_works():
    cleanup()
    assert True
