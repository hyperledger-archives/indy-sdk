#!/usr/bin/env python

# this script runs inside a docker container for testing.

import re
import os
import shutil
import gzip_so_file
import toml_utils


def test_debian_file():
    print('executing: cargo deb')
    os.system('cargo deb --no-build')
    debian_dir = '/sdk/vcx/libvcx/target/debian/'
    pattern = re.compile('lib.*\.deb')
    debian_file = filter(pattern.match, os.listdir(debian_dir))[0]
    print('copying file: %s' % debian_file)
    shutil.copyfile(debian_dir + debian_file, '/data/' + debian_file)

def test_gzip_so_file():
    version = toml_utils.get_version_from_file('/sdk/vcx/libvcx/Cargo.toml')
    src = '/sdk/vcx/libvcx/target/release/libvcx.so.%s' % version
    dest_dir = '/data'
    gzip_so_file.gz(src, dest_dir)

def cargo_commands():
    print('changing directory')
    os.chdir('/sdk/vcx/libvcx')
    print(os.listdir(os.getcwd()))
    print('executing: cargo update-version')
    os.system('cargo update-version')
    print('executing: cargo update-so')
    os.system('cargo update-so')

def main():
    cargo_commands()
    test_debian_file()
    test_gzip_so_file()

if __name__ == '__main__':
    main()
