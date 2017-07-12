from .storage import StorageUtils


def test_storage_utils_indy_home_path_works():
    home_path = StorageUtils.indy_home_path()
    assert '.indy' in str(home_path)


def test_storage_utils_indy_temp_path_works():
    tmp_path = StorageUtils.indy_temp_path()
    assert 'indy' in str(tmp_path)


def test_storage_utils_cleanup_works():
    StorageUtils.cleanup()
