# Development
FROM libindy
ARG uid=1000
RUN useradd -ms /bin/bash -u $uid android
RUN usermod -aG sudo android

RUN apt-get update -y && apt-get install -y \
    openjdk-8-jdk \
    maven
    
# Install Android SDK and NDK 
RUN mkdir -m 777 /home/android/android-sdk-linux
RUN wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip -P /home/android/android-sdk-linux
RUN unzip /home/android/android-sdk-linux/tools_r25.2.3-linux.zip -d /home/android/android-sdk-linux
RUN ls -al /home/android/android-sdk-linux
RUN yes | .//home/android/android-sdk-linux/tools/android update sdk --no-ui
RUN yes | .//home/android/android-sdk-linux/tools/bin/sdkmanager "ndk-bundle"



RUN echo "android ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers 

USER android
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.34.1
ENV PATH /home/android/.cargo/bin:$PATH
