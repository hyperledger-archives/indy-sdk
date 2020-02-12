#!/usr/bin/python3

import os
import sys
import shutil
import json
import tarfile

# Packs the npm project into .tgz
# Then creates a debian package of it.

def create_deb(filename):
    if not os.path.isfile(filename):
        print('%s doesnt exist' % filename)
        sys.exit(1)

    if os.path.isfile('package') or os.path.isdir('package') or os.path.isfile('vcx') or os.path.isdir('vcx'):
        print('file or directory \'package\' or \'vcx\' already exists, cannot perform action.')
        sys.exit(1)

    with tarfile.open(filename, 'r') as f_out:
        f_out.extractall()

    (name, version) = get_info('package')
    shutil.move('package', 'vcx')
    prefix = '/usr/local/lib/node_modules'
    directory_to_package = 'vcx'
    cmd = 'fpm -s dir --output-type deb --name %s --version %s --prefix %s %s' % (name, version, prefix, directory_to_package)
    os.system(cmd)
    shutil.rmtree('vcx')


def print_usage():
    print("USAGE: python create_npm_deb.py TARFILE")


def get_info(dirname):
    with open(dirname+'/package.json', 'r') as f_in:
        o = f_in.read()
    j = json.loads(o)
    return (str(j['name']), str(j['version']))



if __name__ == '__main__':
    if len(sys.argv) != 2:
        print_usage()
        sys.exit(1)
    else:
        dirname = 'package'
        create_deb(sys.argv[1])

