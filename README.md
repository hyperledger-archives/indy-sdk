# rust-libindy-wrapper

[LibIndy](https://github.com/hyperledger/indy-sdk/tree/master/libindy) major artifact of the SDK is a C-callable library that provides the basic building blocks for the creation of applications on the top of Hyperledger Indy, which provides a distributed-ledger-based foundation for self-sovereign identity.

**rust-libindy-wrapper** is a library for assisting developers using LibIndy API.   

**rust-libindy-wrapper** does not include LibIndy.  This will need to be built separately.  See [IndySDK github](https://github.com/hyperledger/indy-sdk)

## using rust-libindy-wrapper

*Assumptions*: LibIndy is installed.  And, you understand the basics.

### Step 1
Add rust-libindy-wrapper to Cargo.toml

```
[dependencies]
rust-libindy-wrapper = "0.2.13"
```

### Step 2
setup an environment variable that points to LibIndy library.
eg:
```
LIBINDY_DIR="/Users/developer/indy-sdk/libindy/target/release"
```

### Step 3
Use **rust-libindy-wrapper**.   For now, best recommendation is to check out the tests.

# License
Released under Apache 2.0 and MIT.  See license files in git repo.
