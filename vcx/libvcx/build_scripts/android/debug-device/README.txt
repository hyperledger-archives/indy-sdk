open a terminal

adb kill-server
#The -writable-system switch is what allows us to remount /system as rw
RUST_BACKTRACE=1 /Users/androidbuild1/Library/Android/sdk/tools/emulator -writable-system -avd Pixel_API_26 -netdelay none -netspeed full
adb start-server
# List attached android devices
adb devices -l


cd sdk/vcx/libvcx/build_scripts/android/debug-device/
adb pull /system/etc/mkshrc .
add the RUST_BACKTRACE environment variable
adb push mkshrc /sdcard/Download/
adb root
adb shell
# getenforce
# cat /proc/mounts|grep -i system
# mount -o rw,remount /system
# cp /sdcard/Download/mkshrc /system/etc/mkshrc

Stop and Restart the emulator
#The -writable-system switch is what allows us to remount /system as rw
RUST_BACKTRACE=1 /Users/androidbuild1/Library/Android/sdk/tools/emulator -writable-system -avd Pixel_API_26 -netdelay none -netspeed full
cd sdk/vcx/libvcx/build_scripts/android/debug-device/
adb logcat > debug-device.out 2>&1 &
tail -f debug-device.out
RUST_BACKTRACE=1 am start -N -n "com.example.lodder.vcxtest/com.example.lodder.vcxtest.MainActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER
ps ax|grep -i logcat
kill -9 <pid>


--------------------------------------------------------------------------------------

1. create Android.mk and java file
2. compile using mmm
3. put the generated jar to target /data/
4. run it:
# export CLASS_PATH=/data/MyTest.jar
# app_process /data com.android.test.MyTest


--------------------------------------------------------------------------------------


adb shell
# mount -o rw,remount /
# cp /sdcard/Download/mkshrc /system/etc/
# setenforce 0
# mount -o rw,remount -t ext4 /dev/block/pci/pci0000:00/0000:00:03.0/by-name/system /system


adb install-multiple -r -t \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/outputs/apk/debug/app-debug.apk

adb install-multiple -r -t \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/resources/instant-run/debug/resources-debug.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_7.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_2.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_1.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/dep/dependencies.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_3.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_8.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_5.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_6.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_4.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_9.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/split-apk/debug/slices/slice_0.apk \
/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/build/intermediates/instant-run-apk/debug/app-debug.apk

adb root
adb unroot

adb shell


------------------------------------------------------------------------------------------------------------------------------------------------


$ rustc -Clinker=./NDK/arm/bin/arm-linux-androideabi-gcc -Car=./NDK/arm/bin/arm-linux-androideabi-ar --target=arm-linux-androideabi -g 1.rs
$ adb push ./1 /sdcard/
$ adb shell
(adb)$ cd /sdcard/
(adb)$ RUST_BACKTRACE=full ./1


cd /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/libvcx/build_scripts/android/debug-device/debugvcx
export PATH=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/NDK/x86/bin:$PATH
cargo build --target i686-linux-android --release --verbose
adb push ./target/i686-linux-android/release/debugvcx /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libvcxall.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libindy.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libcrypto.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libssl.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libsodium.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libvcx.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libzmq.so /sdcard/Download/
adb push /Users/androidbuild1/forge/work/code/evernym/sdk/vcx/wrappers/java/android/vcxtest/app/jni/x86/libz.so /sdcard/Download/

adb root
adb shell
mkdir /data/vcxlib
mv /sdcard/Download/debugvcx /data/vcxlib
mv /sdcard/Download/libvcxall.so /data/vcxlib/
mv /sdcard/Download/libindy.so /data/vcxlib/
mv /sdcard/Download/libcrypto.so /data/vcxlib/
mv /sdcard/Download/libssl.so /data/vcxlib/
mv /sdcard/Download/libsodium.so /data/vcxlib/
mv /sdcard/Download/libvcx.so /data/vcxlib/
mv /sdcard/Download/libzmq.so /data/vcxlib/
mv /sdcard/Download/libz.so /data/vcxlib/
cd /data/vcxlib
chmod a+x debugvcx
RUST_BACKTRACE=full ./debugvcx

------------------------------------------------------------------------------------------------------------------------------------------------

