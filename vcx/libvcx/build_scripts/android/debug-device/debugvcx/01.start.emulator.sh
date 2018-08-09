#!/bin/sh

#Restart the emulator
adb kill-server
sleep 5
#The -writable-system switch is what allows us to remount /system as rw
RUST_BACKTRACE=1 /Users/norm/Library/Android/sdk/emulator/emulator -writable-system -avd Pixel_API_26 -netdelay none -netspeed full > emulator.debug-device.out 2>&1 &
#RUST_BACKTRACE=1 /Users/norm/Library/Android/sdk/emulator/emulator -writable-system -avd Nexus_5X_API_P -netdelay none -netspeed full > emulator.debug-device.out 2>&1 &
adb start-server
adb logcat > logcat.debug-device.out 2>&1 &
adb devices -l
