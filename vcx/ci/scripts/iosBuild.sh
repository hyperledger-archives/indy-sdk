#!/bin/bash

setup() {
    echo "IOS Build"
    echo "Working Directory: ${PWD}"
    brew update

    install_dependency ruby
    install_dependency curl
    install_dependency git

    if [ ! -f /usr/local/bin/java8 ]; then
        echo "Intalling java8"
        brew cask install java8
    fi

    echo $(ls /Users)
    if [ ! -f /Users/jenkins/Library/Android/sdk ]; then
        echo "Installing Android Sdk"
        brew doctor
        brew install android-sdk
    fi

}

install_dependency() {
    DEP=$1
    echo $(ls /usr/local/bin)
    if [ ! -f /usr/local/bin/${DEP} ]; then
        echo "Intalling ${DEP}"
        brew install ${DEP}
    fi
}

setup