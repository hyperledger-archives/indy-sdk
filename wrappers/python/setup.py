from distutils.core import setup

setup(
    name='python3-indy',
    version='1.3.1',
    packages=['indy'],
    url='https://github.com/hyperledger/indy-sdk',
    license='MIT/Apache-2.0',
    author='Vyacheslav Gudkov',
    author_email='vyacheslav.gudkov@dsr-company.com',
    description='This is the official SDK for Hyperledger Indy (https://www.hyperledger.org/projects), which provides a distributed-ledger-based foundation for self-sovereign identity (https://sovrin.org). The major artifact of the SDK is a c-callable library.',
    install_requires=['pytest', 'pytest-asyncio', 'base58'],
    tests_require=['pytest', 'pytest-asyncio', 'base58']
)
