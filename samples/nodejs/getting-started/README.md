# Getting started with the node.js wrapper

* Make sure you have libindy installed. Checkout the guides [here](https://github.com/hyperledger/indy-sdk/tree/master/doc).
    * On Linux, export the `LD_LIBRARY_PATH` environment variable to point to the `libindy.so` parent directory, or copy libindy.so to `/usr/lib/libindy.so`.
    * On Mac OS, you must have `libindy.dylib` at `/usr/local/lib/libindy.dylib` before running npm install.
* Inside `samples/nodejs/`
    * Make sure you have a running ledger with `npm run ledger`. You must have docker installed.
    * Then run `npm install` and `npm start`.
 
### Troubleshooting

* If you get an error on the npm install, make sure that LD\_LIBRARY\_PATH is defined or that the library is in the correct directory. See above.
* If you get an `CommonInvalidState` indy error, try rebuilding the ledger with the `--no-cache` docker flag.
* If you just can't figure out why you are getting an indy error, try it with `RUST_LOG=INFO` to see the rust logs.

See documentation for the wrapper at [npmjs.com](https://www.npmjs.com/package/indy-sdk#installing).

### Ready to start consuming indy-sdk?
This example pulls the wrapper from this repository, but you can download it yourself from the npm registry with `npm i indy-sdk`.
