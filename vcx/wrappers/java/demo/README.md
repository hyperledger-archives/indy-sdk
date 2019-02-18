# VCX Java Wrapper Demo

This is a demo of the Faber and Alice using VCX Java wrapper for VCX library.
VCX is the open-source library on top of Libindy which fully implements the credentials exchange.
This demo code is written using Kotlin language

## PREREQUISITES
**Note** that before you can use this demo you must generate c-callable SDK for libindy,libnullpay and libvcx. And you also need to generate the VCX java wrapper jar file.

## FOR MacOS

### Manually Generate tall the needed libraries and run the example

1. Setup Indy SDK build environment for MacOS by following instructions [here](https://github.com/hyperledger/indy-sdk/blob/master/docs/build-guides/mac-build.md)
2. Once you have done cargo build on libindy, libnullpay and libvcx, it should have created .dylib DLL file under /target/debug folder under each of the library folder
3. Generage VCX jar file by following instructions [here](https://github.com/hyperledger/indy-sdk/tree/master/vcx/wrappers/java#jar)
4. Start the local Sovrin network by following instructions [here](https://github.com/hyperledger/indy-sdk#how-to-start-local-nodes-pool-with-docker)
5. Start the dummy cloud agent by following instructions [here](https://github.com/hyperledger/indy-sdk/blob/master/vcx/dummy-cloud-agent/README.md)
    Note: If there is an error starting the dummy cloud agent due port conflict then update the config file in the dummy cloud directory to update the port number
6. Run Faber using ```./gradlew faber```
7. Copy the connection detail printed on the screen
8. Open another terminal window and run Alice using ```./gradlew alice```
9. On alice terminal window once prompted to enter the connection detail, paste the text you copied and press enter

### Auto generate using ```buildlibs.sh``` and run the example

1. Run ```buildlibs.sh``` to auto generate libs and vcx jar file
2. Start the local Sovrin network by following instructions [here](https://github.com/hyperledger/indy-sdk#how-to-start-local-nodes-pool-with-docker)
3. Start the dummy cloud agent by following instructions [here](https://github.com/hyperledger/indy-sdk/blob/master/vcx/dummy-cloud-agent/README.md)
    Note: If there is an error starting the dummy cloud agent due port conflict then update the config file in the dummy cloud directory to update the port number
4. Run Faber using ```./gradlew faber```
5. Copy the connection detail printed on the screen
6. Open another terminal window and run Alice using ```./gradlew alice```
7. On alice terminal window once prompted to enter the connection detail, paste the text you copied and press enter


## Logging
  By default the logging is OFF. If you want to increase the logging level then use following commands
  - ```./gradlew faber -Pmyargs=loglevel,trace```
  - ```./gradlew alice -Pmyargs=loglevel,trace```


## Troubleshooting
 - Each time you run the dummy cloud agent it produces local wallet under .indy_client folder under your home directory. If you shutdown the dummy cloud agent, you need to delete the all the folder under .indy_client
 - When you run faber test it will delete all the wallets under .indy_client/wallet directory except wallet for the dummy cloud agent
