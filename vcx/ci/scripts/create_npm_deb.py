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
        def is_within_directory(directory, target):
            
            abs_directory = os.path.abspath(directory)
            abs_target = os.path.abspath(target)
        
            prefix = os.path.commonprefix([abs_directory, abs_target])
            
            return prefix == abs_directory
        
        def safe_extract(tar, path=".", members=None, *, numeric_owner=False):
        
            for member in tar.getmembers():
                member_path = os.path.join(path, member.name)
                if not is_within_directory(path, member_path):
                    raise Exception("Attempted Path Traversal in Tar File")
        
            tar.extractall(path, members, numeric_owner=numeric_owner) 
            
        
        safe_extract(f_out)

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

