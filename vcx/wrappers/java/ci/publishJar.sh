#!/usr/bin/env bash

set -e
JAR_FOLDER=artifacts/jar
JAR_VERSION=$(find ${JAR_FOLDER} -type f -name 'com.evernym-vcx-*.jar'| perl -nle 'print $& if m{(?<=vcx-)(.*)(?=.jar)}' | head -1 | awk '{print $1}')
echo "Uploading .jar with version number ==> ${JAR_VERSION}"
cp -v settings.xml ${JAR_FOLDER}
pushd ${JAR_FOLDER}

    mvn -e deploy:deploy-file \
        -Durl='${mavenRepo.url}' \
        -DrepositoryId='${mavenRepo.id}' \
        -Dversion=${JAR_VERSION} \
        -Dname="vcx" \
        -Dfile="com.evernym-vcx-${JAR_VERSION}.jar" \
        -DartifactId="vcx" \
        -Dpackaging="jar" \
        -DgroupId="com.evernym" \
        --settings settings.xml
popd
