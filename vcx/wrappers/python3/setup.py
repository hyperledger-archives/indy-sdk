from setuptools import setup, find_packages
import os

PKG_VERSION = os.environ.get('PACKAGE_VERSION') or '0.4.0'
PKG_NAME = os.environ.get('PACKAGE_NAME') or 'python3-wrapper-vcx'

setup(
    name=PKG_NAME,
    version=PKG_VERSION,
    description='Python 3 wrapper for libcxs',
    long_description='None...for now',
    author='Devin Fisher, Ryan Marsh, Mark Hadley, Doug Wightman',
    author_email='ryan.marsh@evernym.com',
    include_package_data=True,
    packages=find_packages(exclude=['demo', 'tests'])
)
