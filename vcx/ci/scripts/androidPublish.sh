#!/usr/bin/env bash

set -e
AAR_FOLDER=vcx/wrappers/java/artifacts/aar
AAR_VERSION=$(find ${AAR_FOLDER} -type f -name 'com.evernym-vcx_*-release.aar'| perl -nle 'print $& if m{(?<=vcx_)(.*)(?=_x86)}' | head -1 | awk '{print $1}')
echo "Uploading .aar with version number ==> ${AAR_VERSION}"
cp -v settings.xml ${AAR_FOLDER}
pushd ${AAR_FOLDER}

    mvn -e deploy:deploy-file \
        -Durl="https://evernym.mycloudrepo.io/repositories/libvcx-android" \
        -DrepositoryId="io.cloudrepo" \
        -Dversion=${AAR_VERSION} \
        -Dfile="com.evernym-vcx_${AAR_VERSION}_x86-armv7-release.aar" \
        -DartifactId="vcx" \
        -Dpackaging="aar" \
        -DgroupId="com.evernym" \
        --settings settings.xml
popd