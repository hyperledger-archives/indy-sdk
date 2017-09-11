#!groovy

@Library('SovrinHelpers') _

try {
    testing()
    publishing()
    if (acceptanceTesting()) {
        releasing()
    }
    notifyingSuccess()
} catch (err) {
    notifyingFailure()
    throw err
}

def testing() {
    stage('Testing') {
        isDebugTests = !(env.BRANCH_NAME in ['master', 'rc'])
        parallel([
                'ubuntu-test' : { ubuntuTesting(isDebugTests) },
                //FIXME fix and restore 'libindy-redhat-test' : { rhelTesting() }, IS-307
                'windows-test': { windowsTesting(isDebugTests) }
        ])
    }
}

def publishing() {
    stage('Publishing') {
        if (!(env.BRANCH_NAME in ['master', 'rc'])) {
            echo "${env.BRANCH_NAME}: skip publishing"
            return
        }
        echo "${env.BRANCH_NAME}: start publishing"

        publishedVersions = parallel([
                //FIXME fix and restore 'rhel-files'     : { rhelPublishing() }, IS-307
                'ubuntu-files' : { ubuntuPublishing() },
                'windows-files': { windowsPublishing() },
        ])

        version = publishedVersions['ubuntu-files']
        if (publishedVersions['windows-files'] != version) { // FIXME check rhel too, IS-307
            error "platforms artifacts have different versions"
        }
    }
}

def acceptanceTesting() {
    stage('Acceptance testing') {
        if (env.BRANCH_NAME == 'rc') {
            echo "${env.BRANCH_NAME}: acceptance testing"
            if (approval.check("default")) {
                return true
            }
        } else {
            echo "${env.BRANCH_NAME}: skip acceptance testing"
        }
        return false
    }
}

def releasing() {
    stage('Releasing') {
        if (env.BRANCH_NAME == 'rc') {
            publishingRCtoStable()
        }
    }
}

def notifyingSuccess() {
    if (env.BRANCH_NAME == 'master') {
        currentBuild.result = "SUCCESS"
        node('ubuntu-master') {
            sendNotification.success('indy-sdk')
        }
    }
}

def notifyingFailure() {
    currentBuild.result = "FAILED"
    node('ubuntu-master') {
        sendNotification.fail([slack: env.BRANCH_NAME == 'master'])
    }
}

def getBuildPoolVerOptions(pool_type, plenum_ver, anoncreds_ver, node_ver) {
    return "--build-arg=sovrin_stream=${pool_type} --build-arg indy_plenum_ver=${plenum_ver} --build-arg indy_anoncreds_ver=${anoncreds_ver} --build-arg indy_node_ver=${node_ver}"
}

def openPool(env_name, network_name, pool_type, pool_ver, plenum_ver, anoncreds_ver, node_ver) {
    echo "${env_name} Test: Create docker network (${network_name}) for nodes pool and test image"
    sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

    echo "${env_name} Test: Build docker image for nodes pool ver. ${pool_ver}"
    echo "${env_name} Test: Building nodes pool for versions: plenum ${plenum_ver}, anoncreds ${anoncreds_ver}, node ${node_ver}"
    verOptions = getBuildPoolVerOptions(pool_type, plenum_ver, anoncreds_ver, node_ver)
    def poolEnv = dockerHelpers.build("indy_pool_${pool_ver}", 'ci/indy-pool.dockerfile ci',
            "--build-arg pool_ip=10.0.0.2 ${verOptions}")
    echo "${env_name} Test: Run nodes pool"
    return poolEnv.run("--ip=\"10.0.0.2\" --network=${network_name}")
}

def closePool(env_name, network_name, poolInst) {
    echo "${env_name} Test: Cleanup"
    try {
        sh "docker network inspect ${network_name}"
    } catch (error) {
        echo "${env_name} Tests: error while inspect network ${network_name} - ${error}"
    }
    try {
        echo "${env_name} Test: stop pool"
        poolInst.stop()
    } catch (error) {
        echo "${env_name} Tests: error while stop pool ${error}"
    }
    try {
        sh "docker ps --format '{{.ID}}' --filter network=${network_name} | xargs docker rm -f"
    } catch (error) {
        echo "${env_name} Test: error while force clean-up network ${network_name} - ${error}"
    }
    try {
        echo "${env_name} Test: remove pool network ${network_name}"
        sh "docker network rm ${network_name}"
    } catch (error) {
        echo "${env_name} Test: error while delete ${network_name} - ${error}"
    }
    step([$class: 'WsCleanup'])
}

void getSrcVersion() {
    commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()
    version = sh(returnStdout: true, script: "wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/libindy/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | cut -f2 -d '\"'").trim()
    return version
}

def linuxTesting(file, env_name, run_interoperability_tests, network_name, isDebugTests) {
    def poolInst
    try {
        echo "${env_name} Test: Checkout csm"
        checkout scm

        poolInst = openPool(env_name, network_name, 'stable', 'stable33', '1.1.24', '1.0.10', '1.1.33')

        def testEnv

        dir('libindy') {
            echo "${env_name} Test: Build docker image"
            testEnv = dockerHelpers.build('libindy', file)

            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test"
                sh 'chmod -R 777 /home/indy/indy-anoncreds/'

                try {
                    def featuresArgs = ''
                    if (run_interoperability_tests) {
                        featuresArgs = '--features "interoperability_tests"'
                    }

                    def buildType = ''
                    if (!isDebugTests) {
                        buildType = '--release'
                    }

                    testParams = "$buildType $featuresArgs".trim()

                    echo "${env_name} Test: Build"
                    sh "RUST_BACKTRACE=1 cargo test $testParams --no-run"

                    echo "${env_name} Test: Run tests"
                    sh "RUST_BACKTRACE=1 RUST_LOG=trace RUST_TEST_THREADS=1 TEST_POOL_IP=10.0.0.2 cargo test $testParams"
                    /* TODO FIXME restore after xunit will be fixed
                    sh 'RUST_TEST_THREADS=1 cargo test-xunit'
                    */
                }
                finally {
                    /* TODO FIXME restore after xunit will be fixed
                    junit 'test-results.xml'
                    */
                }
            }
        }

        def folder = isDebugTests ? "debug" : "release"

        sh "cp libindy/target/$folder/libindy.so wrappers/java/lib"
        dir('wrappers/java') {
            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test java wrapper"

                sh "RUST_LOG=trace TEST_POOL_IP=10.0.0.2 mvn clean test"
            }
        }

        sh "cp libindy/target/$folder/libindy.so wrappers/python"
        dir('wrappers/python') {
            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test python wrapper"

                sh '''
                    python3.5 -m pip install --user -e .
                    LD_LIBRARY_PATH=./ RUST_LOG=trace TEST_POOL_IP=10.0.0.2 python3.5 -m pytest
                '''
            }
        }
    }
    finally {
        closePool(env_name, network_name, poolInst)
    }
}

def windowsTesting(isDebugTests) {
    node('win2016') {
        stage('Windows Test') {
            echo "Windows Test: Checkout scm"
            checkout scm

            try {
                echo "Windows Test: Run Indy pool"
                poolVerOptions = getBuildPoolVerOptions('stable', '1.1.24', '1.0.10', '1.1.33')
                bat "docker -H $INDY_SDK_SERVER_IP build --build-arg pool_ip=$INDY_SDK_SERVER_IP ${poolVerOptions} -f ci/indy-pool.dockerfile -t indy_pool ci"
                bat "docker -H $INDY_SDK_SERVER_IP run -d --network host --name indy_pool -p 9701-9708:9701-9708 indy_pool"

                dir('libindy') {
                    echo "Windows Test: Download prebuilt dependencies"
                    bat 'wget -O prebuilt.zip "https://repo.evernym.com/libindy/windows/deps/indy-sdk-deps.zip"'
                    bat 'unzip prebuilt.zip -d prebuilt'

                    def buildType = ''
                    if (!isDebugTests) {
                        buildType = '--release'
                    }

                    echo "Windows Test: Build"
                    withEnv([
                            "INDY_PREBUILT_DEPS_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "MILAGRO_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "ZMQPW_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "SODIUM_LIB_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "OPENSSL_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "PATH=$WORKSPACE\\libindy\\prebuilt\\lib;$PATH",
                            "RUST_BACKTRACE=1"
                    ]) {
                        bat "cargo test $buildType --no-run"

                        echo "Windows Test: Run tests"
                        withEnv([
                                "RUST_TEST_THREADS=1",
                                "RUST_LOG=trace",
                                "TEST_POOL_IP=$INDY_SDK_SERVER_IP"
                        ]) {
                            bat "cargo test $buildType"
                        }
                    }
                }

                //TODO wrappers testing

            } finally {
                try {
                    bat "docker -H $INDY_SDK_SERVER_IP stop indy_pool"
                } catch (ignore) {
                }
                try {
                    bat "docker -H $INDY_SDK_SERVER_IP rm indy_pool"
                } catch (ignore) {
                }
                step([$class: 'WsCleanup'])
            }
        }
    }
}

def ubuntuTesting(isDebugTests) {
    node('ubuntu') {
        stage('Ubuntu Test') {
            linuxTesting("ci/ubuntu.dockerfile ci", "Ubuntu", true, "pool_network", isDebugTests)
        }
    }
}

def rhelTesting(isDebugTests) {
    node('ubuntu') {
        stage('RedHat Test') {
            linuxTesting("ci/amazon.dockerfile ci", "RedHat", false, "pool_network", isDebugTests)
        }
    }
}

def rhelPublishing() {
    node('ubuntu') {
        stage('Publish Libindy RPM Files') {
            try {
                echo 'Publish Rpm files: Checkout csm'
                checkout scm

                version = getSrcVersion()

                dir('libindy') {
                    echo 'Publish Rpm: Build docker image'
                    def testEnv = dockerHelpers.build('indy-sdk', 'ci/amazon.dockerfile ci')

                    testEnv.inside('-u 0:0') {

                        sh 'chmod -R 777 ci'

                        withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                            sh "./ci/libindy-rpm-build-and-upload.sh $version $evernym_repo_key $env.BRANCH_NAME $env.BUILD_NUMBER"
                        }
                    }
                }
            }
            finally {
                echo 'Publish RPM: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
    return version
}

def ubuntuPublishing() {
    node('ubuntu') {
        stage('Publish Ubuntu Files') {
            try {
                echo 'Publish Ubuntu files: Checkout csm'
                checkout scm

                version = getSrcVersion()

                echo 'Publish Ubuntu files: Build docker image'
                testEnv = dockerHelpers.build('indy-sdk', 'libindy/ci/ubuntu.dockerfile libindy/ci')

                libindyDebPublishing(testEnv)
                pythonWrapperPublishing(testEnv, false)
                javaWrapperPublishing(testEnv, false)
            }
            finally {
                echo 'Publish Ubuntu files: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }

    return version
}

def windowsPublishing() {
    node('win2016') {
        stage('Publish Libindy Windows Files') {
            try {
                echo 'Publish Windows files: Checkout csm'
                checkout scm

                version = getSrcVersion()

                dir('libindy') {
                    echo "Publish Windows files: Download prebuilt dependencies"
                    bat 'wget -O prebuilt.zip "https://repo.evernym.com/libindy/windows/deps/indy-sdk-deps.zip"'
                    bat 'unzip prebuilt.zip -d prebuilt'

                    echo "Publish Windows files: Build"
                    withEnv([
                            "INDY_PREBUILT_DEPS_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "MILAGRO_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "ZMQPW_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "SODIUM_LIB_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "OPENSSL_DIR=$WORKSPACE\\libindy\\prebuilt",
                            "PATH=$WORKSPACE\\libindy\\prebuilt\\lib;$PATH",
                            "RUST_BACKTRACE=1"
                    ]) {
                        bat "cargo build --release"
                    }

                    withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                        sh "./ci/libindy-win-zip-and-upload.sh $version '${evernym_repo_key}' $env.BRANCH_NAME $env.BUILD_NUMBER"
                    }
                }
            }
            finally {
                echo 'Publish Windows files: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
    return version
}

def libindyDebPublishing(testEnv) {
    dir('libindy') {
        testEnv.inside('-u 0:0') {
            sh 'chmod -R 755 ci/*.sh'

            withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                sh "./ci/libindy-deb-build-and-upload.sh $version $evernym_repo_key $env.BRANCH_NAME $env.BUILD_NUMBER"
                sh "rm -rf debian"
            }
        }
    }
}

def getSuffix(isRelease, target) {
    def suffix
    if (env.BRANCH_NAME == 'master' && !isRelease) {
        suffix = "-dev-$env.BUILD_NUMBER"
    } else if (env.BRANCH_NAME == 'rc') {
        if (isRelease) {
            suffix = ""
        } else {
            suffix = "-rc-$env.BUILD_NUMBER"
        }
    } else {
        error "Publish To ${target}: invalid case: branch ${env.BRANCH_NAME}, isRelease ${isRelease}"
    }
    return suffix
}

def pythonWrapperPublishing(testEnv, isRelease) {
    dir('wrappers/python') {
        def suffix = getSuffix(isRelease, "Pypi")

        testEnv.inside {
            withCredentials([file(credentialsId: 'pypi_credentials', variable: 'credentialsFile')]) {
                sh 'cp $credentialsFile ./'
                sh "sed -i -E \"s/version='([0-9,.]+).*/version='\\1$suffix',/\" setup.py"
                sh '''
                    python3.5 setup.py sdist
                    python3.5 -m twine upload dist/* --config-file .pypirc
                '''
            }
        }
    }
}

def javaWrapperPublishing(testEnv, isRelease) {
    dir('wrappers/java') {
        echo "Publish To Maven Test: Build docker image"
        def suffix = getSuffix(isRelease, "Maven")

        testEnv.inside {
            echo "Publish To Maven Test: Test"

            sh "sed -i -E -e 'H;1h;\$!d;x' -e \"s/<version>([0-9,.]+)</<version>\\1$suffix</\" pom.xml"

            withCredentials([file(credentialsId: 'artifactory-evernym-settings', variable: 'settingsFile')]) {
                sh 'cp $settingsFile .'

                sh "mvn clean deploy -DskipTests --settings settings.xml"
            }
        }
    }
}

def publishingRCtoStable() {
    node('ubuntu') {
        stage('Moving RC artifacts to Stable') {
            try {
                echo 'Moving RC artifacts to Stable: Checkout csm'
                checkout scm

                version = getSrcVersion()

                echo 'Moving RC artifacts to Stable: libindy'
                publishLibindyRCtoStable(version)

                echo 'Moving RC artifacts to Stable: Build docker image for wrappers publishing'
                testEnv = dockerHelpers.build('indy-sdk', 'libindy/ci/ubuntu.dockerfile libindy/ci')
                echo 'Moving RC artifacts to Stable: python wrapper'
                pythonWrapperPublishing(testEnv, true)
                echo 'Moving RC artifacts to Stable: java wrapper'
                javaWrapperPublishing(testEnv, true)
            } finally {
                echo 'Moving RC artifacts to Stable: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
}

def publishLibindyRCtoStable(version) {
    rcFullVersion = "${version}-${env.BUILD_NUMBER}"
    withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'key')]) {
        for (os in ['ubuntu', 'windows']) { //FIXME add rhel IS-307
            src = "/var/repository/repos/libindy/$os/rc/$rcFullVersion/"
            target = "/var/repository/repos/libindy/$os/stable/$version"
            //should not exists
            sh "ssh -v -oStrictHostKeyChecking=no -i '$key' repo@192.168.11.111 '! ls $target'"
            sh "ssh -v -oStrictHostKeyChecking=no -i '$key' repo@192.168.11.111 cp -r $src $target"
        }
    }
}
