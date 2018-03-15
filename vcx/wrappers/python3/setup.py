from setuptools import setup, find_packages
import os
setup(
    name='vcx',
    version=os.environ['VCX_VERSION'],
    description='Wrapper for libcxs',
    long_description='None...for now',
    author='Devin Fisher, Ryan Marsh, Mark Hadley, Doug Wightman',
    author_email='ryan.marsh@evernym.com',
    include_package_data=True,
    packages=find_packages(exclude=['demo', 'tests'])
)
