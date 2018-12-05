alias l='ls -al'

export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1
#for i in `ls -t /usr/local/Cellar/openssl/`; do export OPENSSL_DIR=/usr/local/Cellar/openssl/$i; break; done

export PYTHONPATH=/Users/iosbuild1/forge/work/code/evernym/sdk-evernym/vcx/libvcx/vcx-indy-sdk/wrappers/python:/Users/iosbuild1/forge/work/code/evernym/sdk-evernym/vcx/wrappers/python3:${PYTHONPATH}

export ANDROID_HOME=/Users/iosbuild1/Library/Android/sdk
export PATH=$ANDROID_HOME/platform-tools:$PATH
export PATH=$ANDROID_HOME/tools:$PATH
export PATH=$ANDROID_HOME/tools/bin:$PATH
export ANDROID_NDK=$ANDROID_HOME/ndk-bundle
export ANDROID_NDK_HOME=$ANDROID_NDK

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion

export PATH="$HOME/.cargo/bin:$PATH"

#export PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/local/Cellar/zeromq/4.2.5/lib/pkgconfig:/usr/local/Cellar/libsodium/1.0.12/lib/pkgconfig

#export LD_LIBRARY_PATH=/Users/iosbuild1/forge/work/code/evernym/sdk-evernym/vcx/libvcx/vcx-indy-sdk/libindy/target/debug:${LD_LIBRARY_PATH}
#export DYLD_LIBRARY_PATH=/Users/iosbuild1/forge/work/code/evernym/sdk-evernym/vcx/libvcx/vcx-indy-sdk/libindy/target/debug:${DYLD_LIBRARY_PATH}
