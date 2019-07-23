from setuptools import setup, find_packages

setup(
    name='sbca-indy-wrapper',
    version='1.8.1-pre',
    description='A Python wrapper for Hyperledger\'s Libindy library',
    url='https://github.com/swisscom-blockchain/sbca-indy-wrapper',

    author='Jeremy Roth (Skilletpan)',
    author_email='skilletpan.14@gmail.com',

    packages=find_packages(exclude=['docs', 'test']),

    python_requires='~=3.6',

    classifiers=[
        'Development Status :: 4 - Beta',

        'Intended Audience :: Developers',
        'Topic :: Software Development',
        'Topic :: Software Development :: Libraries',
        'Topic :: Software Development :: Libraries :: Python Modules',
        'Typing :: Typed',

        'License :: OSI Approved :: Apache Software License',

        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3 :: Only',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',

        'Operating System :: OS Independent'
    ],
    zip_safe=False
)
