from setuptools import setup, find_packages
import os
VERSION = 'VCX_VERSION'
PKG_NAME = os.getenv('PACKAGE_NAME') if os.getenv('PACKAGE_NAME') else 'python3-wrapper-vcx'

def test_pkg_name():
    print(PKG_NAME)

def get_version():
    try:
        return os.environ[VERSION]
    except KeyError:
        return '0.1'


setup(
    name=PKG_NAME,
    version=get_version(),
    description='Python 3 wrapper for libcxs',
    long_description='None...for now',
    author='Devin Fisher, Ryan Marsh, Mark Hadley, Doug Wightman',
    author_email='ryan.marsh@evernym.com',
    include_package_data=True,
    packages=find_packages(exclude=['demo', 'tests'])
)
