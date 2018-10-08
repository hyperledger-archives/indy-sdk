# indy

[LibIndy](https://github.com/hyperledger/indy-sdk/tree/master/libindy) major artifact of the SDK is a C-callable library that provides the basic building blocks for the creation of applications on the top of Hyperledger Indy, which provides a distributed-ledger-based foundation for self-sovereign identity.

**indy** is a library for assisting developers using LibIndy API.   

## Using indy
- **indy** does not include LibIndy. Install native "indy" library:
	* Ubuntu:  https://repo.sovrin.org/lib/apt/xenial/
	* Windows: https://repo.sovrin.org/windows/libindy/
	
- Add **indy** to Cargo.toml 
```
[dependencies]
indy = "1.6.7"
```

# Note
This library is currently in experimental state.

# License
Released under Apache 2.0 and MIT.  See license files in git repo.