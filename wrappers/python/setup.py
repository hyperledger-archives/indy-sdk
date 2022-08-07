from distutils.core import setup
import os

with open(os.path.join(os.path.dirname(__file__), 'version.txt'), 'r') as file:
    PKG_VERSION = file.read().rstrip()

TEST_DEPS = [
    'pytest<3.7', 'pytest-asyncio==0.10.0', 'base58'
]

setup(
    name='python3-indy',
    version=PKG_VERSION,
    packages=['indy'],
    url='https://github.com/hyperledger/indy-sdk',
    license='MIT/Apache-2.0',
    author='Hyperledger Indy Contributors',
    author_email='hyperledger-indy@lists.hyperledger.org',
    description='This is the official SDK for Hyperledger Indy (https://www.hyperledger.org/projects), which provides a distributed-ledger-based foundation for self-sovereign identity (https://sovrin.org). The major artifact of the SDK is a c-callable library.',
    install_requires=['base58'],
    tests_require=TEST_DEPS,
    extras_require={
        'test': TEST_DEPS
    },
    include_package_data=True
)
