#!/usr/bin/env bash

set -e
JAR_FOLDER=vcx/wrappers/java/artifacts/jar
JAR_VERSION=$(find ${JAR_FOLDER} -type f -name 'com.evernym-vcx_*.jar'| perl -nle 'print $& if m{(?<=vcx_)(.*)(?=.jar)}' | head -1 | awk '{print $1}')
echo "Uploading .jar with version number ==> ${JAR_VERSION}"
cp -v settings.xml ${JAR_FOLDER}
pushd ${JAR_FOLDER}

    mvn -e deploy:deploy-file \
        -Durl="https://evernym.mycloudrepo.io/repositories/libvcx-java" \
        -DrepositoryId="io.cloudrepo" \
        -Dversion=${JAR_VERSION} \
        -Dfile="com.evernym-vcx_${JAR_VERSION}.jar" \
        -DartifactId="vcx" \
        -Dpackaging="jar" \
        -DgroupId="com.evernym" \
        --settings settings.xml
popd