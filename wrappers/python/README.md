<a href="https://sovrin.org/" target="_blank"><img src="https://avatars2.githubusercontent.com/u/22057628?v=3&s=50" align="right"></a>

## Indy SDK for Python

This is a Python wrapper for [Indy](https://www.hyperledger.org/projects/indy). It is implemented using a foreign function interface (FFI) to a native library written in Rust. Indy is the
open-source codebase behind the Sovrin network for self-sovereign digital identity.

This Python wrapper currently requires python 3.6.

Pull requests welcome!

### How to build

- Clone indy-sdk repo from https://github.com/hyperledger/indy-sdk

- Build the native "indy" library at https://github.com/hyperledger/indy-sdk:

	cargo build

- Then copy the resulting `libindy.so` to `/usr/lib/`.

- cd wrappers/python

- Create virtual env if you want

- install dependencies with pip install

Then run

- Start local nodes pool on 127.0.0.1:9701-9708 with Docker (for now just follow same point in platform-specific instructions for libindy)

- Execute tests with pytest


### Example use
For the main workflow examples check tests in demo folder: https://github.com/hyperledger/indy-sdk/tree/master/wrappers/python/tests/demo
