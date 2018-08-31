#!/bin/sh
gradle clean
gradle build
RUST_BACKTRACE=1 java -cp build/libs/jnavcx.jar:/Users/androidbuild1/.gradle/caches/modules-2/files-2.1/net.java.dev.jna/jna/4.5.1/65bd0cacc9c79a21c6ed8e9f588577cd3c2f85b9/jna-4.5.1.jar -Djna.library.path=/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/libvcx/target/x86_64-apple-darwin/release com.evernym.sdk.vcx.VcxProvisionAsync
