#!/usr/bin/env python

import sys
import gzip
import shutil
import os
import tarfile

# Given a list of files, this will create a tar.gz archive.


def create_tar(source, tfile):
	tar = tarfile.open(tfile, "w:gz")
	try:
		for f in source:
			tar.add(f, arcname=os.path.basename(f))
	except IOError:
		print("error archiving %s"%f)
	else:
		tar.close()

	
def gz(source, dest_dir):
    src_file = os.path.basename(source)
    dest_dir = dest_dir.rstrip('/')
    
    z = dest_dir + '/' + src_file + '.gz'
    with open(source, 'r') as f_in, gzip.open(z, 'wb' ) as f_out:
        shutil.copyfileobj(f_in, f_out)


def print_usage():
	print("USAGE: python %s FILE [FILE..] TAR"% sys.argv[0])
	print("exiting...")
	sys.exit(1)


if __name__ == "__main__":
	if len(sys.argv) < 3:
		print_usage()
	else:
		tfile = sys.argv[len(sys.argv)-1]
		files = sys.argv[1:len(sys.argv)-1]

		# check that the tarfile doesnt already exist
		if os.path.isfile(tfile):
			print("Tarfile already exists")
			print_usage()
		
		# check that all arguments are valid files:
		for i in files:
			if not os.path.isfile(i):
				print('File %s does not exist.'%i)
				print_usage()
	
		create_tar(files, tfile)
		
