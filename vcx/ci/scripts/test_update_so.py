#!/usr/bin/env python
import toml_utils
import version_utils
import os

TOML = 'Cargo.toml'
SO_FILE = 'libvcx.so'
def test_update_so_file():
    toml_utils.update_so(SO_FILE, '4.5.6')
    cmd = "ls libvcx*"
    os.system(cmd)
    cmd = "mv libvcx* %s" % SO_FILE
    os.system(cmd)
    print('finished')

def test_get_version_from_toml():
    b = toml_utils.get_version_from_file(TOML)
    print('Version is: %s' % b)


test_get_version_from_toml()
test_update_so_file()


#test_update_so('libvcx.so', 1, 2, 3456)
