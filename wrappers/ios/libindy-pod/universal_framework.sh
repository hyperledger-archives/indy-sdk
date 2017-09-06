#!/bin/sh


exec > /tmp/${PROJECT_NAME}_archive.log 2>&1

UNIVERSAL_OUTPUTFOLDER=${BUILD_DIR}/${CONFIGURATION}-universal

if [ "true" == ${ALREADYINVOKED:-false} ]
then
echo "RECURSION: Detected, stopping"
else
export ALREADYINVOKED="true"

# make sure the output directory exists
mkdir -p "${UNIVERSAL_OUTPUTFOLDER}"

if [ ${PLATFORM_NAME} = "iphonesimulator" ]
then
xcodebuild -workspace "${WORKSPACE_PATH}" -scheme "${TARGET_NAME}" -configuration ${CONFIGURATION} -sdk iphoneos  ENABLE_BITCODE=YES OTHER_CFLAGS="-fembed-bitcode" BITCODE_GENERATION_MODE=bitcode clean build
else

xcodebuild -workspace "${WORKSPACE_PATH}" -scheme "${TARGET_NAME}" -configuration ${CONFIGURATION} -sdk iphonesimulator -destination 'platform=iOS Simulator,name=iPhone 6' ONLY_ACTIVE_ARCH=NO ARCHS='i386 x86_64' BUILD_DIR="${BUILD_DIR}" BUILD_ROOT="${BUILD_ROOT}" ENABLE_BITCODE=YES OTHER_CFLAGS="-fembed-bitcode" BITCODE_GENERATION_MODE=bitcode clean build
fi

if [ -d "${BUILD_DIR}/${CONFIGURATION}-iphoneos/${TARGET_NAME}.framework/${TARGET_NAME}" ]; then
echo "iphoneos exists 1"
fi


# Step 1. Copy the framework structure (from iphoneos build) to the universal folder
echo "Copying to output folder"
cp -R "${TARGET_BUILD_DIR}/${FULL_PRODUCT_NAME}" "${UNIVERSAL_OUTPUTFOLDER}/"

# Step 2. Copy Swift modules from iphonesimulator build (if it exists) to the copied framework directory

if [ -d "${BUILD_DIR}/${CONFIGURATION}-iphoneos/${TARGET_NAME}.framework/${TARGET_NAME}" ]; then
echo "iphoneos exists 2"
fi

echo "will copy files"

# Step 2. Copy Swift modules from iphonesimulator build (if it exists) to the copied framework directory
SIMULATOR_SWIFT_MODULES_DIR="${BUILD_DIR}/${CONFIGURATION}-iphonesimulator/${TARGET_NAME}.framework/Modules/${TARGET_NAME}.swiftmodule/."
if [ -d "${SIMULATOR_SWIFT_MODULES_DIR}" ]; then
cp -R "${SIMULATOR_SWIFT_MODULES_DIR}" "${UNIVERSAL_OUTPUTFOLDER}/${TARGET_NAME}.framework/Modules/${TARGET_NAME}.swiftmodule"
fi

# Step 3. Create universal binary file using lipo and place the combined executable in the copied framework directory
echo "Combining executables"

open "${UNIVERSAL_OUTPUTFOLDER}/${TARGET_NAME}.framework/"

lipo -create -output "${UNIVERSAL_OUTPUTFOLDER}/${TARGET_NAME}.framework/${TARGET_NAME}" "${BUILD_DIR}/${CONFIGURATION}-iphonesimulator/${TARGET_NAME}.framework/${TARGET_NAME}" "${BUILD_DIR}/${CONFIGURATION}-iphoneos/${TARGET_NAME}.framework/${TARGET_NAME}"

# Step 4. Create universal binaries for embedded frameworks
#for SUB_FRAMEWORK in $( ls "${UNIVERSAL_OUTPUTFOLDER}/${TARGET_NAME}.framework/Frameworks" ); do
#BINARY_NAME="${SUB_FRAMEWORK%.*}"
#lipo -create -output "${UNIVERSAL_OUTPUTFOLDER}/${TARGET_NAME}.framework/Frameworks/${SUB_FRAMEWORK}/${BINARY_NAME}" "${BUILD_DIR}/${CONFIGURATION}-iphonesimulator/${SUB_FRAMEWORK}/${BINARY_NAME}" "$${TARGET_BUILD_DIR}/${TARGET_NAME}.framework/Frameworks/${SUB_FRAMEWORK}/${BINARY_NAME}"
#done

# Step 5. Convenience step to copy the framework to the project's directory
echo "Copying to project dir"


yes | cp -Rf "${UNIVERSAL_OUTPUTFOLDER}/${FULL_PRODUCT_NAME}" "${PROJECT_DIR}"

open "${PROJECT_DIR}"

fi



