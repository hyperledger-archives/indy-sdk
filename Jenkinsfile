#!groovy​

@Library('SovrinHelpers') _

name = 'indy-sdk'
def err
def publishBranch = (env.BRANCH_NAME == 'master' || env.BRANCH_NAME == 'devel')

try {

// ALL BRANCHES: master, devel, PRs

    // 1. TEST
    stage('Test') {

        def tests = [:]

        tests['ubuntu-test'] = {
            node('ubuntu') {
                stage('Ubuntu Test') {
                    testUbuntu()
                }
            }
        }

        tests['redhat-test'] = {
            node('ubuntu') {
                stage('RedHat Test') {
                    testRedHat()
                }
            }
        }

        tests['ubuntu-java-test'] = {
            node('ubuntu') {
                stage('Ubuntu Java Test') {
                    javaTestUbuntu()
                }
            }
        }

        tests['ubuntu-python-test'] = {
            node('ubuntu') {
                stage('Ubuntu Python Test') {
                    pythonTestUbuntu()
                }
            }
        }

        parallel(tests)
    }

    if (!publishBranch) {
        echo "${env.BRANCH_NAME}: skip publishing"
        return
    }

    stage('Publish') {

        def publishSteps = [:]

        // 2. PUBLISH TO Cargo
        publishSteps['cargo'] = {
            node('ubuntu') {
                stage('Publish to Cargo') {
                     publishToCargo()
                }
            }
        }

        // 3. PUBLISH RPMS TO repo.evernym.com
        publishSteps['rpm-files'] = {
            node('ubuntu') {
                stage('Publish RPM Files') {
                     publishRpmFiles()
                }
            }
        }

        // 4. PUBLISH DEB TO repo.evernym.com
        publishSteps['deb-files'] = {
            node('ubuntu') {
                stage('Publish DEB Files') {
                     publishDebFiles()
                }
            }
        }

        parallel(publishSteps)
    }
} catch (e) {
    currentBuild.result = "FAILED"
    node('ubuntu-master') {
        sendNotification.fail([slack: publishBranch])
    }
    err = e
} finally {
    if (err) {
        throw err
    }
    currentBuild.result = "SUCCESS"
    if (publishBranch) {
        node('ubuntu-master') {
            sendNotification.success(name)
        }
    }
}

def testPipeline(file, env_name, run_interoperability_tests) {
    def poolInst
    def network_name = "pool_network"
    try {
        echo "${env_name} Test: Checkout csm"
        checkout scm

        echo "${env_name} Test: Create docker network (${network_name}) for nodes pool and test image"
        sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

        echo "${env_name} Test: Build docker image for nodes pool"
        def poolEnv = dockerHelpers.build('indy_pool', 'ci/indy-pool.dockerfile ci')
        echo "${env_name} Test: Run nodes pool"
        poolInst = poolEnv.run("--ip=\"10.0.0.2\" --network=${network_name}")

        echo "${env_name} Test: Build docker image"
        def testEnv = dockerHelpers.build(name, file)

        testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
           echo "${env_name} Test: Test"
           sh 'chmod -R 777 /home/indy/'
           sh 'cargo update'

           try {
                if (run_interoperability_tests) {
                    sh 'RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --features "interoperability_tests"'
                }
                else {
                    sh 'RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test'
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
    finally {
        echo "${env_name} Test: Cleanup"
        try {
            sh "docker network inspect ${network_name}"
        } catch (err) {
            echo "${env_name} Tests: error while inspect network ${network_name} - ${err}"
        }
        try {
            echo "${env_name} Test: stop pool"
            poolInst.stop()
        } catch (err) {
            echo "${env_name} Tests: error while stop pool ${err}"
        }
        try {
            echo "${env_name} Test: remove pool network ${network_name}"
            sh "docker network rm ${network_name}"
        } catch (err) {
            echo "${env_name} Test: error while delete ${network_name} - ${err}"
        }
        step([$class: 'WsCleanup'])
    }
}

def testUbuntu() {
    testPipeline("ci/ubuntu.dockerfile ci", "Ubuntu", true)
}

def testRedHat() {
    testPipeline("ci/amazon.dockerfile ci", "RedHat", false)
}

def javaTestUbuntu() {
    def poolInst
    def network_name = "pool_network"
    try {
        echo 'Ubuntu Java Test: Checkout csm'
        checkout scm

        echo "Ubuntu Java Test: Create docker network (${network_name}) for nodes pool and test image"
        sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

        echo 'Ubuntu Java Test: Build docker image for nodes pool'
        def poolEnv = dockerHelpers.build('indy_pool', 'ci/indy-pool.dockerfile ci')
        echo 'Ubuntu Java Test: Run nodes pool'
        poolInst = poolEnv.run("--ip=\"10.0.0.2\" --network=${network_name}")

        echo 'Ubuntu Java Test: Build docker image'
        def testEnv = dockerHelpers.build(name, 'ci/java.dockerfile ci')

        testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
            echo 'Ubuntu Java Test: Test'

            sh '''
                cd wrappers/java
                mvn clean test
            '''
        }
    }
    finally {
        echo 'Ubuntu Java Test: Cleanup'
        try {
            sh "docker network inspect ${network_name}"
        } catch (ignore) {
        }
        try {
            if (poolInst) {
                echo 'Ubuntu Java Test: stop pool'
                poolInst.stop()
            }
        } catch (err) {
            echo "Ubuntu Java Tests: error while stop pool ${err}"
        }
        try {
            echo "Ubuntu Java Test: remove pool network ${network_name}"
            sh "docker network rm ${network_name}"
        } catch (err) {
            echo "Ubuntu Java Test: error while delete ${network_name} - ${err}"
        }
        step([$class: 'WsCleanup'])
    }
}

def pythonTestUbuntu() {
    def poolInst
    def network_name = "pool_network"
    try {
        echo 'Ubuntu Python Test: Checkout csm'
        checkout scm

        echo "Ubuntu Python Test: Create docker network (${network_name}) for nodes pool and test image"
        sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

        echo 'Ubuntu Python Test: Build docker image for nodes pool'
        def poolEnv = dockerHelpers.build('indy_pool', 'ci/indy-pool.dockerfile ci')
        echo 'Ubuntu Python Test: Run nodes pool'
        poolInst = poolEnv.run("--ip=\"10.0.0.2\" --network=${network_name}")

        echo 'Ubuntu Python Test: Build docker image'
        def testEnv = dockerHelpers.build(name, 'ci/python.dockerfile ci')

        testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
            echo 'Ubuntu Python Test: Test'

            sh '''
                cd wrappers/python
                python3.6 -m pip install -e .
                python3.6 -m pytest
            '''
        }
    }
    finally {
        echo "Ubuntu Python Test: Cleanup"
        try {
            sh "docker network inspect ${network_name}"
        } catch (err) {
            echo "Ubuntu Python Tests: error while inspect network ${network_name} - ${err}"
        }
        try {
            echo "Ubuntu Python Test: stop pool"
            poolInst.stop()
        } catch (err) {
            echo "Ubuntu Python Tests: error while stop pool ${err}"
        }
        try {
            echo "Ubuntu Python Test: remove pool network ${network_name}"
            sh "docker network rm ${network_name}"
        } catch (err) {
            echo "Ubuntu Python Test: error while delete ${network_name} - ${err}"
        }
        step([$class: 'WsCleanup'])
    }
}

def publishToCargo() {
    try {
        echo 'Publish to Cargo: Checkout csm'
        checkout scm

        echo 'Publish to Cargo: Build docker image'
        def testEnv = dockerHelpers.build(name)

        testEnv.inside {
            echo 'Update version'

            def suffix = env.BRANCH_NAME == 'master' ? env.BUILD_NUMBER : 'devel-' + env.BUILD_NUMBER;

            sh 'chmod -R 777 ci'
            sh "ci/update-package-version.sh $suffix"

            withCredentials([string(credentialsId: 'cargoSecretKey', variable: 'SECRET')]) {
                sh 'cargo login $SECRET'
            }

            sh 'cargo package --allow-dirty'

            sh 'cargo publish --allow-dirty'
        }
    }
    finally {
        echo 'Publish to cargo: Cleanup'
        step([$class: 'WsCleanup'])
    }
}

def publishRpmFiles() {
    try {
        echo 'Publish Rpm files: Checkout csm'
        checkout scm

        echo 'Publish Rpm: Build docker image'
        def testEnv = dockerHelpers.build(name, 'ci/amazon.dockerfile ci')

        testEnv.inside('-u 0:0') {

            commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()

            sh 'chmod -R 777 ci'

            withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                sh "./ci/rpm-build-and-upload.sh $commit $evernym_repo_key"
            }
        }
    }
    finally {
        echo 'Publish RPM: Cleanup'
        step([$class: 'WsCleanup'])
    }
}

def publishDebFiles() {
    try {
        echo 'Publish Deb files: Checkout csm'
        checkout scm

        echo 'Publish Deb: Build docker image'
        def testEnv = dockerHelpers.build(name)

        testEnv.inside('-u 0:0') {

            commit = sh(returnStdout: true, script: 'git rev-parse HEAD').trim()

            sh 'chmod -R 777 ci'

            withCredentials([file(credentialsId: 'EvernymRepoSSHKey', variable: 'evernym_repo_key')]) {
                sh "./ci/deb-build-and-upload.sh $commit $evernym_repo_key"
            }
        }
    }
    finally {
        echo 'Publish Deb: Cleanup'
        step([$class: 'WsCleanup'])
    }
}