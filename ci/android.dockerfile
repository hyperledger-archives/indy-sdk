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

# VCX USER 
RUN useradd -ms /bin/bash -u $uid vcx
RUN usermod -aG sudo vcx

# Install Android SDK and NDK 
RUN mkdir -m 777 /home/vcx/android-sdk-linux
RUN wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip -P /home/vcx/android-sdk-linux
RUN unzip /home/vcx/android-sdk-linux/tools_r25.2.3-linux.zip -d /home/vcx/android-sdk-linux
RUN ls -al /home/vcx/android-sdk-linux
RUN yes | .//home/vcx/android-sdk-linux/tools/android update sdk --no-ui
RUN yes | .//home/vcx/android-sdk-linux/tools/bin/sdkmanager "ndk-bundle"

USER vcx 
# cargo deb for debian packaging of libvcx
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

