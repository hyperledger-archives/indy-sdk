#!groovy​

def success = true

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

    if (env.BRANCH_NAME != 'master' && env.BRANCH_NAME != 'devel') {
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
    success = false
    currentBuild.result = "FAILED"
    throw e
} finally {
    if (success && (env.BRANCH_NAME == 'master' || env.BRANCH_NAME == 'devel')) {
        currentBuild.result = "SUCCESS"
    }
}

def testUbuntu() {
    try {
        echo 'Ubuntu Test: Checkout csm'
        checkout scm

        echo 'Ubuntu Test: Build docker image'
        def testEnv = docker.build 'sovrin-client-rust-test'

        testEnv.inside("-u root"){
            echo 'Ubuntu Test: Test'
            sh 'cd /home/sorvin-client-rust'

            try {
                sh 'cargo test-xunit'
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
        def testEnv = docker.build 'sovrin-client-rust-test'

        testEnv.inside("-u root"){
            sh 'cd /home/sorvin-client-rust'

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