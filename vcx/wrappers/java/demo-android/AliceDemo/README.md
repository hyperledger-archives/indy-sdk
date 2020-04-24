# Running the Alice Android Demo
This demo project is for the Alice on Android simulator or actual devices. You can use any Faber demo in different wrappers for testing. Internally, the application serializes and deserializes the vcx connection object between operations. It saves the configuration details in the shared preference, and use this when it is available for initialization of VCX.
## Prerequisites

#### Android Studio
It requires the Android Studio 3.6 or newer

#### NDK
Open the demo project and install the NDK (Side by side) version 20.0.5594570 using Android Studio
```
Tools -> SDK Manager -> SDK Tools -> Check 'Show Package Details' -> Install the NDK (Side by side) version 20.0.5594570
```

#### Native Libraries
You need to create a jniLibs folder (`app/source/main/jniLibs`) on Android Studio and include following native libraries for each ABI. Alternatively, you can download jniLibs folder from [sktston/vcx-skeleton-android](https://github.com/sktston/vcx-skeleton-android/tree/master/app/src/main/jniLibs) (This repo assumes your dev platform is MacOS for `libc++_shared.so` file)
- [libindy](https://repo.sovrin.org/android/libindy/stable/)
- [libvcx](https://repo.sovrin.org/android/libvcx/stable/)
- [libnullpay](https://repo.sovrin.org/android/libnullpay/stable/)
- [libjnidispatch v4.5.2](https://github.com/java-native-access/jna/tree/4.5.2/lib/native): You can exract `libjnidispatch.so` from `jar` file using, for example `unzip android-x86.jar libjnidispatch.so` command. Alternatively, you may get a file in the local gradle folder after opening the project in the Android Studio (You can get a location of files in the Android Studio > Project tab > expand External Libraries > expand `net.java.dev.jna:jna:4.5.2@aar` > right click on `classes.jar` > Reveal in Finder > they are under `jni` folder)
- libc++_shared r20: If your platform is MacOS You can get libc++_shared.so file for each ABI in the `~/Library/Android/sdk/ndk/20.0.5594570/sources/cxx-stl/llvm-libc++/libs` folder after instlling NDK in the previous step.

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