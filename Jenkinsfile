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

        //Libindy ubuntu tests
        tests['ubuntu-test'] = {
            node('ubuntu') {
                stage('Ubuntu Test') {
                    testUbuntu()
                }
            }
        }

        //Libindy red hat tests
        tests['redhat-test'] = {
            node('ubuntu') {
                stage('RedHat Test') {
                    testRedHat()
                }
            }
        }

        //Java wrapper ubuntu tests
        tests['ubuntu-java-test'] = {
            node('ubuntu') {
                stage('Ubuntu Java Test') {
                    javaTestUbuntu()
                }
            }
        }

        //Python wrapper ubuntu tests
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

def openPool(env_name, network_name){
    echo "${env_name} Test: Create docker network (${network_name}) for nodes pool and test image"
    sh "docker network create --subnet=10.0.0.0/8 ${network_name}"

    echo "${env_name} Test: Build docker image for nodes pool"
    def poolEnv = dockerHelpers.build('indy_pool', 'ci/indy-pool.dockerfile ci')
    echo "${env_name} Test: Run nodes pool"
    return poolEnv.run("--ip=\"10.0.0.2\" --network=${network_name}")
}

def closePool(env_name, network_name, poolInst){
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

def testPipeline(file, env_name, run_interoperability_tests, network_name) {
    def poolInst
    try {
        echo "${env_name} Test: Checkout csm"
        checkout scm

        poolInst = openPool(env_name, network_name)

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
        closePool(env_name, network_name, poolInst)
    }
}

def testUbuntu() {
    testPipeline("ci/ubuntu.dockerfile ci", "Ubuntu", true, "pool_network")
}

def testRedHat() {
    testPipeline("ci/amazon.dockerfile ci", "RedHat", false, "pool_network")
}

def javaTestUbuntu() {
    def poolInst
    def network_name = "pool_network"
    def env_name = "Ubuntu Java"

    try {
        echo "${env_name} Test: Checkout csm"
        checkout scm

        poolInst = openPool("Ubuntu Java", network_name)

        echo "${env_name} Test: Build docker image"
        def testEnv = dockerHelpers.build(name, 'ci/java.dockerfile ci')

        testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
            echo "${env_name} Test: Test"

            sh '''
                cd wrappers/java
                mvn clean test
            '''
        }
    }
    finally {
        closePool(env_name, network_name, poolInst)
    }
}

def pythonTestUbuntu() {
    def poolInst
    def network_name = "pool_network"
    def env_name = "Ubuntu Python"
    try {
        echo "${env_name} Test: Checkout csm"
        checkout scm

        poolInst = openPool(env_name, network_name)

        echo "${env_name} Test: Build docker image"
        def testEnv = dockerHelpers.build(name, 'ci/python.dockerfile ci')

        testEnv.inside("--ip=\"10.0.0.3\" --network=${network_name}") {
            echo "${env_name} Test: Test"

            sh '''
                cd wrappers/python
                python3.6 -m pip install -e .
                python3.6 -m pytest
            '''
        }
    }
    finally {
        closePool(env_name, network_name, poolInst)
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

            sh 'chmod -R 777 ci'
            sh "ci/update-package-version.sh $env.BUILD_NUMBER"

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
                sh "./ci/rpm-build-and-upload.sh $commit $evernym_repo_key $env.BUILD_NUMBER"
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
                sh "./ci/deb-build-and-upload.sh $commit $evernym_repo_key $env.BUILD_NUMBER"
            }
        }
    }
    finally {
        echo 'Publish Deb: Cleanup'
        step([$class: 'WsCleanup'])
    }
}