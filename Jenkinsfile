#!groovy

@Library('SovrinHelpers') _

try {
    testing()
    publishing()
    notifyingSuccess()
} catch (err) {
    notifyingFailure(err)
}

def testing() {
    stage('Testing') {
        parallel([
                'libindy-ubuntu-test': { libindyUbuntuTesting() },
                'libindy-redhat-test': { libindyRedHatTesting() },
                'java-ubuntu-test'   : { javaWrapperUbuntuTesting() },
                'python-ubuntu-test' : { pythonWrapperUbuntuTesting() }
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
                'liblindy-to-cargo': { publishingLibindyToCargo() },
                'libindy-rpm-files': { publishingLibindyRpmFiles() },
                'libindy-deb-files': { publishLibindyDebFiles() }
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
    throw err
}

def openPool(env_name, network_name) {
    echo "${env_name} Test: Create docker network (${network_name}) for nodes pool and test image"
    sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

    echo "${env_name} Test: Build docker image for nodes pool"
    def poolEnv = dockerHelpers.build('indy_pool', 'ci/indy-pool.dockerfile ci', '--build-arg pool_ip=10.0.0.2')
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

        sh "cp -r ci libindy"

        dir('libindy') {
            poolInst = openPool(env_name, network_name)

            echo "${env_name} Test: Build docker image"
            def testEnv = dockerHelpers.build('libindy', file)

            testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                echo "${env_name} Test: Test"
                sh 'chmod -R 777 /home/indy/'
                sh 'cargo update'

                try {
                    if (run_interoperability_tests) {
                        sh 'RUST_BACKTRACE=1 RUST_TEST_THREADS=1 TEST_POOL_IP=10.0.0.2 cargo test --features "interoperability_tests"'
                    } else {
                        sh 'RUST_BACKTRACE=1 RUST_TEST_THREADS=1 TEST_POOL_IP=10.0.0.2 cargo test'
                    }
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
    }
    finally {
        closePool(env_name, network_name, poolInst)
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

def javaWrapperUbuntuTesting() {
    node('ubuntu') {
        stage('Ubuntu Java Test') {
            def poolInst
            def network_name = "pool_network"
            def env_name = "Ubuntu Java"

            try {
                echo "${env_name} Test: Checkout csm"
                checkout scm

                sh "cp -r ci wrappers/java"

                dir('wrappers/java') {
                    poolInst = openPool("Ubuntu Java", network_name)

                    echo "${env_name} Test: Build docker image"
                    def testEnv = dockerHelpers.build('java-indy-sdk', 'ci/java.dockerfile ci')

                    testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                        echo "${env_name} Test: Test"

                        sh "mvn clean test"
                    }
                }
            }
            finally {
                closePool(env_name, network_name, poolInst)
            }
        }
    }
}

def pythonWrapperUbuntuTesting() {
    node('ubuntu') {
        stage('Ubuntu Python Test') {
            def poolInst
            def network_name = "pool_network"
            def env_name = "Ubuntu Python"
            try {
                echo "${env_name} Test: Checkout csm"
                checkout scm

                sh "cp -r ci wrappers/python"

                dir('wrappers/python') {

                    poolInst = openPool(env_name, network_name)

                    echo "${env_name} Test: Build docker image"
                    def testEnv = dockerHelpers.build('python-indy-sdk', 'ci/python.dockerfile ci')

                    testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
                        echo "${env_name} Test: Test"

                        sh '''
                            python3.6 -m pip install -e .
                            python3.6 -m pytest
                        '''
                    }
                }
            }
            finally {
                closePool(env_name, network_name, poolInst)
            }
        }
    }
}

def publishingLibindyToCargo() {
    node('ubuntu') {
        stage('Publish to Cargo') {
            try {
                echo 'Publish to Cargo: Checkout csm'
                checkout scm

                sh "cp -r ci libindy"

                dir('libindy') {
                    echo 'Publish to Cargo: Build docker image'
                    def testEnv = dockerHelpers.build('indy-sdk')

                    testEnv.inside {
                        echo 'Update version'

                        sh 'chmod -R 777 ci'
                        sh "ci/libindy-update-package-version.sh $env.BUILD_NUMBER"

                        withCredentials([string(credentialsId: 'cargoSecretKey', variable: 'SECRET')]) {
                            sh 'cargo login $SECRET'
                        }

                        sh 'cargo package --allow-dirty'

                        sh 'cargo publish --allow-dirty'
                    }
                }
            }
            finally {
                echo 'Publish to cargo: Cleanup'
                step([$class: 'WsCleanup'])
            }
        }
    }
}

def publishingLibindyRpmFiles() {
    node('ubuntu') {
        stage('Publish RPM Files') {
            try {
                echo 'Publish Rpm files: Checkout csm'
                checkout scm

                sh "cp -r ci libindy"

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

def publishLibindyDebFiles() {
    node('ubuntu') {
        stage('Publish DEB Files') {
            try {
                echo 'Publish Deb files: Checkout csm'
                checkout scm

                sh "cp -r ci libindy"
                sh "cp -r debian libindy"

                commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()

                dir('libindy') {
                    echo 'Publish Deb: Build docker image'
                    def testEnv = dockerHelpers.build('indy-sdk')

                    testEnv.inside('-u 0:0') {

                        sh 'chmod -R 777 ci'

                        withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                            sh "./ci/libindy-deb-build-and-upload.sh $commit $evernym_repo_key $env.BUILD_NUMBER"
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
