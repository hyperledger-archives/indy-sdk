#!/usr/bin/env python

import sys
import gzip
import shutil
import os

# will gzip the so file


def gz(source, dest_dir):
    src_file = os.path.basename(source)
    dest_dir = dest_dir.rstrip('/')
    
    z = dest_dir + '/' + src_file + '.gz'
    with open(source, 'r') as f_in, gzip.open(z, 'wb' ) as f_out:
        shutil.copyfileobj(f_in, f_out)

def print_usage():
    print("USAGE: python gzip_so_file.py FILE DEST_DIR")
    print("exiting...")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print_usage()
    else:
        if os.path.isfile(sys.argv[1]) and os.path.exists(sys.argv[2]):
            gz(sys.argv[1], sys.argv[2])
        else:
            print('Parameters are invalid, file and dest directory both must exists')
            print_usage()
