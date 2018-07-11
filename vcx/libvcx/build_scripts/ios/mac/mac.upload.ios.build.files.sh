#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

DATETIME=$1
if [ "$DATETIME" = "" ]; then
    echo "You must pass the datetime as the first parameter to the script. (i.e. 20180522.1527 - YYYYmmdd.hhMM)"
    exit 1
fi

# cd $VCX_SDK/vcx/wrappers/ios/vcx/lib
# mv libvcx.a libvcx.a.original
# cp -v libvcxall.a libvcx.a
# rm libvcx.a_${DATETIME}_universal.tar.gz
# tar vzcf libvcx.a_${DATETIME}_universal.tar.gz libvcx.a
# mv libvcx.a.original libvcx.a
# curl --insecure -u normjarvis -X POST -F file=@$VCX_SDK/vcx/wrappers/ios/vcx/lib/libvcx.a_${DATETIME}_universal.tar.gz https://kraken.corp.evernym.com/repo/ios/upload
# # Download the file at https://repo.corp.evernym.com/filely/ios/libvcx.a_${DATETIME}_universal.tar.gz
# cp $VCX_SDK/vcx/wrappers/ios/vcx/lib/libvcx.a_${DATETIME}_universal.tar.gz  /usr/local/var/www/download/ios

# 1) open /Users/iosbuild1/forge/work/code/evernym/sdk/vcx/wrappers/ios/vcx/vcx.xcodeproj in xcode
# 2) Select vcx as the target in Xcode
# 3) Select generic iOS device as Build only device
# 4) Select the menu Product -> archive
# 5) If every thing compiled successfully then folder with `vcx.framework` will be opened 
# 6) Now upload iOS .tar.gz and .zip files from the build as assets to servers...
#    Just run the script /Users/iosbuild1/forge/work/code/evernym/sdk/vcx/libvcx/build_scripts/ios/mac/mac.upload.ios.build.files.sh

cd $VCX_SDK/vcx/wrappers/ios/vcx
mv lib/libvcx.a lib/libvcx.a.original
cp -v lib/libvcxall.a lib/libvcx.a
xcodebuild -project vcx.xcodeproj -scheme vcx -configuration Debug CONFIGURATION_BUILD_DIR=. clean > $START_DIR/xcodebuild.vcx.framework.build.out 2>&1
xcodebuild -project vcx.xcodeproj -scheme vcx -configuration Debug -sdk iphonesimulator CONFIGURATION_BUILD_DIR=. build >> $START_DIR/xcodebuild.vcx.framework.build.out 2>&1
mv vcx.framework vcx.framework.iphonesimulator
xcodebuild -project vcx.xcodeproj -scheme vcx -configuration Debug -sdk iphoneos CONFIGURATION_BUILD_DIR=. build >> $START_DIR/xcodebuild.vcx.framework.build.out 2>&1
lipo -create -output combined.ios.vcx vcx.framework/vcx vcx.framework.iphonesimulator/vcx
mv combined.ios.vcx vcx.framework/vcx
mv lib/libvcx.a.original lib/libvcx.a
rm -rf vcx.framework.iphonesimulator

#cd $VCX_SDK/vcx/wrappers/ios/vcx
#cp -v lib/libnullpay.a vcx.framework/lib/libnullpay.a

mkdir -p vcx.framework/lib
cp -v lib/libvcxall.a vcx.framework/lib/libvcx.a
mkdir -p vcx.framework/Headers
cp -v ConnectMeVcx.h vcx.framework/Headers
cp -v include/libvcx.h vcx.framework/Headers
cp -v vcx/vcx.h vcx.framework/Headers
rm -rf $VCX_SDK/vcx/wrappers/ios/vcx/tmp
mkdir -p $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx/
cp -rvp vcx.framework $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx/
cd $VCX_SDK/vcx/wrappers/ios/vcx/tmp
rm vcx.framework_${DATETIME}_universal.zip
zip -r vcx.framework_${DATETIME}_universal.zip vcx
# |---vcx.framework_20180522.1635_universal.zip
# |---vcx
#      |---vcx.framework
#            |----lib
#            |       |---libvcx.a
#            |----headers
#            |       |---vcx.h
#            |       |---ConnectMeVcx.h
#            |       |---libvcx.h
#            |----vcx
#            |----Modules
#            |       |---module.modulemap
#            |----Info.plist
#curl --insecure -u normjarvis -X POST -F file=@./vcx.framework_${DATETIME}_universal.zip https://kraken.corp.evernym.com/repo/ios/upload
# Download the file at https://repo.corp.evernym.com/filely/ios/vcx.framework_${DATETIME}_universal.zip
#sudo cp ./vcx.framework_${DATETIME}_universal.zip  /usr/local/var/www/download/ios
