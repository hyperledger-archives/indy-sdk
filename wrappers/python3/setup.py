from setuptools import setup, find_packages
import os
VERSION = 'VCX_VERSION'


def get_version():
    if os.environ[VERSION] is None:
        return '0.1'
    else:
        return os.environ[VERSION]


setup(
    name='vcx',
    version=get_version(),
    description='Wrapper for libcxs',
    long_description='None...for now',
    author='Devin Fisher, Ryan Marsh, Mark Hadley, Doug Wightman',
    author_email='ryan.marsh@evernym.com',
    include_package_data=True,
    packages=find_packages(exclude=['demo', 'tests'])
)
