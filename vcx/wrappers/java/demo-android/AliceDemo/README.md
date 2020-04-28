# Running the Alice Android Demo
This demo project is for the Alice on Android simulator or actual devices. You can use any Faber demo in different wrappers for testing. Internally, the application serializes and deserializes the vcx connection object between operations. It saves the configuration details in the shared preference, and uses this when it is available for initialization of VCX.
## Prerequisites

#### Android Studio
It requires the Android Studio 3.6 or newer

#### Create Native Libraries
Run the script. This will download all native libraries needed for this project, and create the jniLib folder with required ABIs
```
$ ./populate_libraries.sh
``` 

**Note**: You can change the version numbers for `libindy` and `libvcx` in the script `populate_libraries.sh`

## Native Libraries included
All libraries will be available after running the script, but if you want to get those libraries by yourself, please refer to the below.

- [libindy stable v1.15.0](https://repo.sovrin.org/android/libindy/stable/1.15.0/)
- [libvcx stable v0.8.0](https://repo.sovrin.org/android/libvcx/stable/0.8.0/)
- [libjnidispatch v4.5.2](https://github.com/java-native-access/jna/tree/4.5.2/lib/native): You can extract `libjnidispatch.so` from `jar` file using, for example `unzip android-x86.jar libjnidispatch.so` command. Alternatively, you may get a file in the local gradle folder (You can get a location of file in the Android Studio > Project tab > expand External Libraries > expand `net.java.dev.jna:jna:4.5.2@aar` > right click on classes.jar > Reveal in Finder > they are under `jni` folder)
- [libc++_shared r21b](https://developer.android.com/ndk/downloads): If your platform is macOS, and downloaded the latest NDK, you can get libc++_shared.so file for each ABI in the `~/Library/Android/sdk/ndk/21.1.6352462/sources/cxx-stl/llvm-libc++/libs` folder

## Steps to run Demo

#### Cloud Agent
You would like to start [Dummy Cloud Agent](https://github.com/hyperledger/indy-sdk/tree/c09fcd538b7cab41acc38b0c31e1afd7e1dc87b4/vcx/dummy-cloud-agent) in the remote host rather than localhost, or you need to modify the `serviceEndPoint` of invitation from Faber to 10.0.2.2 which is the localhost of the Android simulator. 

Update the `agncy_url` field in the `app/src/main/res/raw/provision_config.json` file with your cloud agent's url

#### Indy Pool
You would also like to start the [Indy Pool](https://github.com/hyperledger/indy-sdk#how-to-start-local-nodes-pool-with-docker) on a specific IP address with the same reason in the cloud agent. Alternatively, you may use some public Indy Pools available on the web. 

Update `app/src/main/res/raw/genesis_txn.txt` file with the genesis transaction info.

#### Run the Alice Demo
1. Run the Faber with a different demo application
1. Click the `PROVISION` button to provision an agent, and initialize VCX. 
1. Copy the invitation from the Faber, and paste it in the Invitation field of the Alice Demo Application
1. Click the `CONNECTION REQUEST` button
1. After connection established, issue credential from Faber demo
1. Click the `ACCEPT OFFER` button in the Alice Demo Application, you will get a credential in a moment
1. In the Faber Demo, ask for proof request
1. Click the `PRESENT PROOF` button. Faber will verify the proof and send the ack after that. 
1. Alice Demo Application will get an ack, and you are done.