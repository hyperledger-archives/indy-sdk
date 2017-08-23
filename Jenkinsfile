#!groovy

@Library('SovrinHelpers') _

try {
    testing()
    publishing()
    notifyingSuccess()
} catch (err) {
    notifyingFailure(err)
    throw err
}

def testing() {
    stage('Testing') {
        parallel([
                'libindy-ubuntu-test' : { libindyUbuntuTesting() },
                //FIXME fix and restore 'libindy-redhat-test' : { libindyRedHatTesting() }, IS-307
                'libindy-windows-test': { libindyWindowsTesting() }
        ])
    }
}

def publishing() {
    stage('Publishing') {
        if (env.BRANCH_NAME != 'master') {
            echo "${env.BRANCH_NAME}: skip publishing"
            return
        }

        parallel([
                //FIXME fix and restore 'libindy-rpm-files'     : { publishingLibindyRpmFiles() }, IS-307
                'libindy-deb-files'     : { publishingLibindyDebFiles() },
                'libindy-win-files'     : { publishingLibindyWinFiles() },
                'python-wrapper-to-pipy': { publishingPythonWrapperToPipy() }
        ])
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

def notifyingFailure(err) {
    currentBuild.result = "FAILED"
    node('ubuntu-master') {
        sendNotification.fail([slack: env.BRANCH_NAME == 'master'])
    }
}

def openPool(env_name, network_name, pool_ver, plenum_ver, anoncreds_ver, node_ver) {
    echo "${env_name} Test: Create docker network (${network_name}) for nodes pool and test image"
    sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

    echo "${env_name} Test: Build docker image for nodes pool ver. ${pool_ver}"
    echo "${env_name} Test: Building nodes pool for versions: plenum ${plenum_ver}, anoncreds ${anoncreds_ver}, node ${node_ver}"
    def poolEnv = dockerHelpers.build("indy_pool_${pool_ver}", 'ci/indy-pool.dockerfile ci',
            "--build-arg pool_ip=10.0.0.2 --build-arg indy_plenum_ver=${plenum_ver} --build-arg indy_anoncreds_ver=${anoncreds_ver} --build-arg indy_node_ver=${node_ver}")
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

def libindyTest(file, env_name, run_interoperability_tests, network_name) {
    def poolInst
    try {
        echo "${env_name} Test: Checkout csm"
        checkout scm

        poolInst = openPool(env_name, network_name, '105', '1.0.95', '1.0.25', '1.0.105')

        def testEnv

        dir('libindy') {
            echo "${env_name} Test: Build docker image"
            testEnv = dockerHelpers.build('libindy', file)

            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test"
                sh 'chmod -R 777 /home/indy/indy-anoncreds/'

                try {
                    def features_args = ""
                    if (run_interoperability_tests) {
                        features_args = '--features "interoperability_tests"'
                    }
                    echo "${env_name} Test: Build"
                    sh "RUST_BACKTRACE=1 cargo test $features_args --no-run"

                    echo "${env_name} Test: Run tests"
                    sh "RUST_BACKTRACE=1 RUST_LOG=trace RUST_TEST_THREADS=1 TEST_POOL_IP=10.0.0.2 cargo test $features_args"
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

        sh "cp libindy/target/debug/libindy.so wrappers/java/lib"
        dir('wrappers/java') {
            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test java wrapper"

                sh "RUST_LOG=trace TEST_POOL_IP=10.0.0.2 mvn clean test"
            }
        }

        sh "cp libindy/target/debug/libindy.so wrappers/python"
        dir('wrappers/python') {
            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test python wrapper"

                sh '''
                    python3.6 -m pip install -e .
                    LD_LIBRARY_PATH=./ RUST_LOG=trace TEST_POOL_IP=10.0.0.2 python3.6 -m pytest
                '''
            }
        }
    }
    finally {
        closePool(env_name, network_name, poolInst)
    }
}

def libindyWindowsTesting() {
    node('win2016') {
        stage('Windows Test') {
            echo "Windows Test: Checkout scm"
            checkout scm

            try {
                echo "Windows Test: Run Indy pool"
                bat "docker -H $INDY_SDK_SERVER_IP build --build-arg pool_ip=$INDY_SDK_SERVER_IP -f ci/indy-pool.dockerfile -t indy_pool ci"
                bat "docker -H $INDY_SDK_SERVER_IP run -d --network host --name indy_pool -p 9701-9708:9701-9708 indy_pool"

                dir('libindy') {
                    echo "Windows Test: Download prebuilt dependencies"
                    bat 'wget -O prebuilt.zip "https://repo.evernym.com/libindy/windows/deps/indy-sdk-deps.zip"'
                    bat 'unzip prebuilt.zip -d prebuilt'

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
                        bat "cargo test --no-run"
                    }

                    echo "Windows Test: Run tests"
                    withEnv([
                            "RUST_TEST_THREADS=1",
                            "RUST_LOG=trace",
                            "RUST_BACKTRACE=1",
                            "TEST_POOL_IP=$INDY_SDK_SERVER_IP"
                    ]) {
                        bat "cargo test"
                    }
                }
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

def libindyUbuntuTesting() {
    node('ubuntu') {
        stage('Ubuntu Test') {
            libindyTest("ci/ubuntu.dockerfile ci", "Ubuntu", true, "pool_network")
        }
    }
}

def libindyRedHatTesting() {
    node('ubuntu') {
        stage('RedHat Test') {
            libindyTest("ci/amazon.dockerfile ci", "RedHat", false, "pool_network")
        }
    }
}

def publishingLibindyRpmFiles() {
    node('ubuntu') {
        stage('Publish Libindy RPM Files') {
            try {
                echo 'Publish Rpm files: Checkout csm'
                checkout scm

                commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()

                dir('libindy') {
                    echo 'Publish Rpm: Build docker image'
                    def testEnv = dockerHelpers.build('indy-sdk', 'ci/amazon.dockerfile ci')

                    testEnv.inside('-u 0:0') {

                        sh 'chmod -R 777 ci'

                        withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                            sh "./ci/libindy-rpm-build-and-upload.sh $commit $evernym_repo_key $env.BUILD_NUMBER"
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
}

def publishingLibindyDebFiles() {
    node('ubuntu') {
        stage('Publish Libindy DEB Files') {
            try {
                echo 'Publish Deb files: Checkout csm'
                checkout scm

                commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()

                dir('libindy') {
                    echo 'Publish Deb: Build docker image'
                    def testEnv = dockerHelpers.build('indy-sdk')

                    testEnv.inside('-u 0:0') {

                        sh 'chmod -R 777 ci'

                        withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                            sh "./ci/libindy-deb-build-and-upload.sh $commit $evernym_repo_key $env.BUILD_NUMBER"
                            sh "rm -rf debian"
                        }
                    }
                }
            }
            finally {
                echo 'Publish Deb: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
}

def publishingLibindyWinFiles() {
    node('win2016') {
        stage('Publish Libindy Windows Files') {
            try {
                echo 'Publish Windows files: Checkout csm'
                checkout scm

                commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()

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
                        sh "./ci/libindy-win-zip-and-upload.sh $commit '${evernym_repo_key}' $env.BUILD_NUMBER"
                    }
                }
            }
            finally {
                echo 'Publish Windows files: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
}

def publishingPythonWrapperToPipy() {
    node('ubuntu') {
        stage('Publish Python Wrapper To Pipy') {
            try {
                echo 'Publish Deb files: Checkout csm'
                checkout scm

                dir('wrappers/python') {
                    echo 'Publish Deb: Build docker image'
                    def testEnv = dockerHelpers.build('python-indy-sdk', 'ci/python.dockerfile ci')

                    testEnv.inside {

                        withCredentials([file(credentialsId: 'pypi_credentials', variable: 'credentialsFile')]) {
                            sh 'cp $credentialsFile ./'

                            sh "chmod -R 777 ci"
                            sh "ci/python-wrapper-update-package-version.sh $env.BUILD_NUMBER"

                            sh '''
                                python3.6 setup.py sdist
                                python3.6 -m twine upload dist/* --config-file .pypirc
                            '''
                        }
                    }
                }
            }
            finally {
                echo 'Publish Deb: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
}
