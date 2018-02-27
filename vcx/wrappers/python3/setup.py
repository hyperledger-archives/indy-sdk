from setuptools import setup, find_packages


with open('README.md') as f:
    readme = f.read()

setup(
    name='vcx',
    version='0.1.0',
    description='Wrapper for libcxs',
    long_description=readme,
    author='Devin Fisher',
    author_email='devin.fisher@evernym.com',
    packages=find_packages(exclude=('tests', 'docs')),
    include_package_data=True
)
