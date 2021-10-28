ARG WRAPPER_BASE_IMAGE
FROM ${WRAPPER_BASE_IMAGE}

RUN apt-get update && apt-get install openjdk-8-jdk maven python3-distutils python3 jq zip -y
ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

ENV ANDROID_BUILD_FOLDER=/tmp/android_build
ENV ANDROID_SDK=${ANDROID_BUILD_FOLDER}/sdk
ENV ANDROID_SDK_ROOT=${ANDROID_SDK}
ENV ANDROID_HOME=${ANDROID_SDK}
ENV TOOLCHAIN_PREFIX=${ANDROID_BUILD_FOLDER}/toolchains/linux
ENV ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/android-ndk-r20
ENV PATH=${PATH}:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/tools:${ANDROID_HOME}/tools/bin
