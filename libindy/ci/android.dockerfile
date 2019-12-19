FROM libindy-test
# to see base image for this one see this file: libindy/ci/ubuntu.dockerfile. It is build in CI/CD pipelines
ENV ANDROID_BUILD_FOLDER=/tmp/android_build
ENV ANDROID_SDK=${ANDROID_BUILD_FOLDER}/sdk
ENV ANDROID_SDK_ROOT=${ANDROID_SDK}
ENV ANDROID_HOME=${ANDROID_SDK}
ENV TOOLCHAIN_PREFIX=${ANDROID_BUILD_FOLDER}/toolchains/linux
ENV ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/android-ndk-r20
ENV PATH=${PATH}:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/tools:${ANDROID_HOME}/tools/bin

COPY android.prepare.sh .
COPY setup.android.env.sh .
USER root
RUN chmod +x android.prepare.sh
RUN chown indy:indy android.prepare.sh
USER indy
RUN ["./android.prepare.sh", "subarch"]
