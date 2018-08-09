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

