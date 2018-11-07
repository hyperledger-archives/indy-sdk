# NodeJS samples for Indy SDK

Each sample can be run individually with `node <filename>`.

Some of samples require validators running. They expect the validators to be running either at localhost (127.0.0.1) or at an IP configured in the environment variable `TEST_POOL_IP`. There is a validator pool that can be used which resides at `doc/getting-started/docker-compose.yml`, which runs at IP 10.0.0.2.

## Getting started with the node.js wrapper

* Make sure you have libindy installed. Checkout the guides [here](https://github.com/hyperledger/indy-sdk/tree/master/doc).
    * On Linux, export the `LD_LIBRARY_PATH` environment variable to point to the `libindy.so` parent directory, or copy libindy.so to `/usr/lib/libindy.so`.
    * On Mac OS, you must have `libindy.dylib` at `/usr/local/lib/libindy.dylib` before running npm install.
    
## Run this samples

* Inside `samples/nodejs/samples/`
    * For some samples, make sure you have a running ledger with `npm run ledger`. You must have docker installed.
    * Then run `npm install` to install all NodeJS dependencies 
    * And then `node <filename>` to run a specific sample
 
### Troubleshooting

* If you get an error on the npm install, make sure that LD\_LIBRARY\_PATH is defined or that the library is in the correct directory. See above.
* If you get an `CommonInvalidState` indy error, try rebuilding the ledger with the `--no-cache` docker flag.
* If you just can't figure out why you are getting an indy error, try it with `RUST_LOG=INFO` to see the rust logs.

See documentation for the wrapper at [npmjs.com](https://www.npmjs.com/package/indy-sdk#installing).

