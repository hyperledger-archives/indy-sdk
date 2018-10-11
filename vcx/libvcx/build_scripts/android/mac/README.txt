Build android libvcx.a from source code
===============================================================
download https://gradle.org/next-steps/?version=3.4.1&format=bin
extract the zip to /Users/norm/forge/tools/
sudo mkdir /opt/gradle
sudo ln -s /Users/norm/forge/tools/gradle-3.4.1 /opt/gradle/gradle-3.4.1
cd /Users/norm/forge/work/code/evernym/sdk-evernym
vi ./vcx/ci/scripts/androidBuild.sh
and then change "wget" to "wget --no-check-certificate" and save the file
also you may need to change "curl" to "curl --insecure" and save the file
OR
run sudo ./vcx/ci/scripts/installCert.sh
LIBINDY_VERSION="1.6.5" LIBINDY_BRANCH="stable" ./vcx/ci/scripts/androidBuild.sh x86; \
LIBINDY_VERSION="1.6.5" LIBINDY_BRANCH="stable" ./vcx/ci/scripts/androidBuild.sh x86_64; \
LIBINDY_VERSION="1.6.5" LIBINDY_BRANCH="stable" ./vcx/ci/scripts/androidBuild.sh arm; \
LIBINDY_VERSION="1.6.5" LIBINDY_BRANCH="stable" ./vcx/ci/scripts/androidBuild.sh armv7; \
LIBINDY_VERSION="1.6.5" LIBINDY_BRANCH="stable" ./vcx/ci/scripts/androidBuild.sh arm64
chmod a+x ./vcx/ci/scripts/androidPackage.sh
vi ./vcx/ci/scripts/androidPackage.sh
and then change /home/vcx/android-sdk-linux
to /Users/norm/Library/Android/sdk and save the file
./vcx/ci/scripts/androidPackage.sh
When it is successful then the .aar file is located at ./vcx/wrappers/java/android/build/outputs/aar
cd ./vcx/wrappers/java/android/build/outputs/aar
mvn install:install-file -Dfile=com.evernym-vcx_[new_version]_x86-armv7-release.aar -DgroupId=com.evernym -DartifactId=vcx -Dversion=[new_version] -Dpackaging=aar
change new_version to something like, i.e. 1.0.0-20-08-2018T19-32
now you can cd to the checked out connectme source code and change the android/app/build.gradle
to have the line -- compile 'com.evernym:vcx:[new_version]@aar'
Now do a make clean;make cleancache;make pre-run
Now do a make run-android to launch the connectme app in the android emulator



Steps to build libindy.so and libvcx.so for android
when you have NOT built them before on this machine
---------------------------------------------------------------------------
1) Login to a new account on a macOS High Sierra (10.13.4) computer.
ASSUMING THE LOGIN NAME IS: androidbuild1
2) Make sure that the oracle java SDK is installed on the mac
3) Checkout the sdk project using https://github.com/evernym/sdk.git or git@github.com:evernym/sdk.git
4) Copy the sdk/vcx/libvcx/build_scripts/android/mac/.bash_profile file to your home directory /Users/[username] and replace the
username androidbuild1 with your username.
5) Re-start your terminal/iterm so that the bash settings take effect
6) Install Android Studio or start Android Studio to make sure that the Android sdk
is installed at /Users/[username]/Library/Android/sdk
7) Startup a terminal and cd into sdk/vcx/libvcx/build_scripts/android/mac
8) Run the script ./mac.01.libindy.setup.sh (make sure the brew install commands are successful)
9) Restart your terminal for environment variables to take effect and cd into sdk/vcx/libvcx/build_scripts/android/mac
10) Run the script 'source ./mac.02.libindy.env.sh'
11) Run the script ./mac.03.libindy.build.sh
12) Run the script ./mac.06.libvcx.build.sh (Test failures do not prevent the .so files from being correctly built)
13) Run the script ./mac.08.copy.shared.libs.to.app.sh
14) Run the script ./mac.09.combine.shared.libs.sh
15) Now you should be able to open up the project sdk/vcx/wrappers/java/android/vcxtest
in Android Studio and build and run the android app in the simulator


Steps to re-build libindy.so and libvcx.so for android
when you have ALREADY built them before on this machine
--------------------------------------------------------------------------
1) Startup a terminal and cd into sdk/vcx/libvcx/build_scripts/android/mac
2) Run the script 'source ./mac.02.libindy.env.sh'
3) Run the script ./mac.03.libindy.build.sh
4) Run the script ./mac.06.libvcx.build.sh (Test failures do not prevent the .so files from being correctly built)
5) Run the script ./mac.08.copy.shared.libs.to.app.sh
6) Run the script ./mac.09.combine.shared.libs.sh
7) Now you should be able to open up the project sdk/vcx/wrappers/java/android/vcxtest
in Android Studio and build and run the android app in the simulator with the latest changes in libindy and libvcx


Steps to automatically re-build libindy.so and libvcx.so for android
every day using the macOS launchd daemon
when you have ALREADY built them before on this machine
--------------------------------------------------------------------------
1) Change the value of the Program key in the launchd.daemon.build.android.libvxc.plist file to the location of the launchd.daemon.build.android.libvxc.sh script on your machine
2) Change the value of the UserName key to the username of the user who will run the script
3) Change the username androidbuild1 to the username of the user who will run the script
4) To see if it is already loaded do: sudo launchctl list|grep local.build_android_libvcx
5) If it is not already loaded then do:
   a) sudo cp sdk/vcx/libvcx/build_scripts/android/mac/launchd.daemon.build.android.libvxc.plist /Library/LaunchDaemons
   b) sudo launchctl load /Library/LaunchDaemons/launchd.daemon.build.android.libvxc.plist
6) Now the building of libvcx will happen automatically once a day at the time listed in the launchd.daemon.build.android.libvxc.plist file
7) To unload the script so that it will not run do: sudo launchctl unload /Library/LaunchDaemons/launchd.daemon.build.android.libvxc.plist
8) To start the job immediately rather than wait until the time listed in the launchd.daemon.build.android.libvxc.plist file do
   sudo launchctl start local.build_android_libvcx

