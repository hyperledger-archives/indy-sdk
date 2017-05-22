#!groovy​

@Library('SovrinHelpers') _

name = 'sovrin-client-rust'
def err
def publishBranch = (env.BRANCH_NAME == 'master' || env.BRANCH_NAME == 'devel')

try {

// ALL BRANCHES: master, devel, PRs

    // 1. TEST
    stage('Test') {
        parallel 'ubuntu-test':{
            node('ubuntu') {
                stage('Ubuntu Test') {
                    testUbuntu()
                }
            }
        }
    }

    if (!publishBranch) {
        echo "${env.BRANCH_NAME}: skip publishing"
        return
    }

    // 2. PUBLISH TO Cargo
    stage('Publish to Cargo') {
        node('ubuntu') {
            publishToCargo()
        }
    }

} catch(e) {
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

def testUbuntu() {
    try {
        echo 'Ubuntu Test: Checkout csm'
        checkout scm

        echo 'Ubuntu Test: Create network for nodes pool and test image'
        try {
            sh 'docker network rm pool-network'
        } finally {
            sh 'docker network create --subnet=10.0.0.0/8 pool_network'
        }

        echo 'Ubuntu Test: Build docker image for nodes pool'
        def poolEnv = dockerHelpers.build('sovrin_pool', 'ci/sovrin-pool.dockerfile ci')
        echo 'Ubuntu Test: Run nodes pool'
        poolEnv.run('--ip="10.0.0.2" --network=pool_network sovrin_pool-test')

        echo 'Ubuntu Test: Build docker image'
        def testEnv = dockerHelpers.build(name)

        testEnv.inside('--ip="10.0.0.3" --network=pool_network sovrin_pool') {
            echo 'Ubuntu Test: Test'

            sh 'cargo update'

            try {
                sh 'RUST_TEST_THREADS=1 cargo test-xunit --features=local_nodes_pool'
            }
            finally {
                junit 'test-results.xml'
            }
        }
    }
    finally {
        echo 'Ubuntu Test: Cleanup'
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