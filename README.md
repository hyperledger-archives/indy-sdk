# rust-indy-sdk-wrapper

IndySDK is the official SDK for Hyperledger Indy, which provides a distributed-ledger-based foundation for self-sovereign identity. 

**rust-indy-sdk-wrapper** is a library for assisting developers using IndySDK API.   

**rust-indy-sdk-wrapper** does not include IndySDK.  This will need to be built separately.  See [IndySDK github](https://github.com/hyperledger/indy-sdk)

## using rust-indy-sdk-wrapper

*Assumptions*: IndySDK is installed.  And, you understand the basics.

### Step 1
Add rust-indy-sdk-wrapper to Cargo.toml

```
[dependencies]
rust-indy-sdk-wrapper = "0.2.11"
```

### Step 2
Use **rust-indy-sdk-wrapper**.   For now, best recommendation is to check out the tests.

