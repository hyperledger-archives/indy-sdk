# Development
FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update -y && apt-get install -y \
    wget \
    sudo \
    curl \
    vim \
    zip \
    unzip \
    git \
    libtool \
    libzmq3-dev \
    python3 \
    openjdk-8-jdk

# Install Gradle
RUN wget https://services.gradle.org/distributions/gradle-3.4.1-bin.zip
RUN mkdir /opt/gradle
RUN unzip -d /opt/gradle gradle-3.4.1-bin.zip

# Indy USER
RUN useradd -ms /bin/bash -u $uid indy
RUN usermod -aG sudo indy

# Install Android SDK and NDK
RUN mkdir -m 777 /home/indy/android-sdk-linux
RUN wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip -P /home/indy/android-sdk-linux
RUN unzip /home/indy/android-sdk-linux/tools_r25.2.3-linux.zip -d /home/indy/android-sdk-linux
RUN ls -al /home/indy/android-sdk-linux
RUN yes | .//home/indy/android-sdk-linux/tools/android update sdk --no-ui
RUN y | .//home/indy/android-sdk-linux/tools/bin/sdkmanager "ndk-bundle"

USER indy
# cargo deb for debian packaging of libindy
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

