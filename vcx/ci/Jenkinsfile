#!groovy

def rust
def vcx_python
def libindy
def vcx_nodejs

testing()

def testing() {
    stage('Testing') {
        parallel([
            'Main' : { mainUbuntu() },
            'Android': { android() },
            'iOS' : { ios() }
        ])
    }
}

def getUserUid() {
    return sh(returnStdout: true, script: 'id -u').trim()
}

def build(name, file, context='.', customParams = '') {
    return docker.build("$name", "${customParams} --build-arg uid=${getUserUid()} -f $file $context")
}

def mainUbuntu() {
    node('ubuntu') {
        stage('Main Build - Ubuntu') {
            try {
                checkout scm
                rust = build('rust', 'vcx/ci/ubuntu.dockerfile', 'vcx')

                // update the versions in the toml file and package.json
                updateVersions(rust)

                // build and test the so file
                buildRust(rust)

                // test rust
                testRust(rust)

                // update the so file to have version
                updateSo(rust)

                // image used as an intermediary, where the libvcx.so is placed into /usr/lib.
                // This image must be built after the rust build and tests have been run.
                libindy = build('libindy', 'vcx/ci/Dockerfile-libindy', 'vcx')

                // test python wrapper, the image must be built after the libindy image build, and must be --no-cache
                vcx_python = build('vcx-python', 'vcx/wrappers/python3/ci/Dockerfile-python-wrapper', 'vcx', '--no-cache')
                testPythonWrapper(vcx_python)

                // test nodejs wrapper, the image must be built after  the libindy image build, and must be --no-cache
                vcx_nodejs = build('vcx-nodejs', 'vcx/wrappers/node/ci/Dockerfile-nodejs-wrapper', 'vcx', '--no-cache')
                testNodeWrapper(vcx_nodejs)

                if (env.BRANCH_NAME == "master") {
                    // create the debian of the library (just the libvcx.so file)
                    createDeb(rust)

                    // create the npm deb
                    createNpmDeb(rust, "*_amd64.tgz")
                    renameAndUploadNpmTgz(rust, "vcx")

                    // create pip installable artifact
                    createPtyhonArtifact(vcx_python, getVcxVersion(vcx_python))
                }

            } catch (Exception ex) {
                currentBuild.result = "FAILED"
                if (env.BRANCH_NAME == "master") {
                    fail()
                }
                echo "$ex error"
            } finally {
                step([$class: 'WsCleanup'])
            }
        }
    }
}


def android() {
    node('ubuntu') {
        stage('Android Build') {
            try {
                checkout scm

                //Build android docker image
                rust = build('rust', 'vcx/ci/android.dockerfile', 'vcx')
                
                //Build .so files for arm64, x86, and arm
                buildAndroid(rust)

                //Package .so files into .aar 
                packageAndroid(rust)

                if (env.BRANCH_NAME == "master") {
                    //Publish package on aptly
                    publishAndroid(rust)
                }
            } catch (Exception ex) {
                // We currently do not fail the main build if the android and ios builds fai
                // currentBuild.result = "FAILED"
                // if (env.BRANCH_NAME == "master") {
                    // fail()
                // }
                echo "$ex error"
            } finally {
                step([$class: 'WsCleanup'])
            }

        }
    }
}

def ios() {
    node('macos-vcx') {
        stage('iOS Build') {
            try {
                checkout scm
                SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
                WORK_DIR = "/Users/jenkins"

                // Build ios architectures
                sh "source vcx/ci/scripts/iosBuild.sh"

                // Publish to Kraken
                TIME_VERSION=sh (script: 'echo $(date +%Y%m%d_%H%M%S)', returnStdout: true ).trim()
                sh "mkdir ${TIME_VERSION}"
                sh "zip -r ${TIME_VERSION}/libvcxall_${TIME_VERSION}.zip vcx/wrappers/ios/vcx/lib/libvcxall.a"

                if (env.BRANCH_NAME == "master") {
                    withCredentials([usernameColonPassword(credentialsId: 'jenkins-kraken-svc', variable: 'KRAKEN_CREDENTIALS')]) {
                        sh "find ${TIME_VERSION} -type f -name 'libvcxall_${TIME_VERSION}.zip' -exec curl -u \"${KRAKEN_CREDENTIALS}\" -X POST  https://kraken.corp.evernym.com/repo/ios/upload -F 'file=@{}' \\;"
                    }
                }
            } catch (Exception ex) {
                // We currently do not fail the main build if the android and ios builds fail
                // currentBuild.result = "FAILED"
                // if (env.BRANCH_NAME == "master") {
                //     fail()
                // }
                echo "$ex error"
            } finally {
                step([$class: 'WsCleanup'])
            }

        }
    }
}

def getRevisionFromGit() {
    revision = sh 'git log --pretty=format:\'%h\' -n 1'
    return revision
}

def addCert(envn) {
    envn.inside {
        CERT_SCRIPT = 'vcx/ci/scripts/getCert.sh'
        // get Evernym Certificate onto machine
        sh "${CERT_SCRIPT}"
    }
}

def uploadPrivateNpm(envn) {
    envn.inside {
        withCredentials([usernameColonPassword(credentialsId: 'jenkins-kraken-svc', variable: 'KRAKEN_CREDENTIALS')]) {
            // upload npm module
            sh "find vcx/wrappers/node/ -type f -name 'vcx-*.tgz' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/npm/upload -F 'file=@{}' \\;"
        }
    }
}
// This runs a python script that extracts version information out of the Cargo.toml file.
def getVcxVersion(envn){
    envn.inside {
        withEnv(["PYTHONPATH=/scripts"]) {
            version = sh(
                script: "python3 -c 'import toml_utils; print(toml_utils.get_version_from_file(\"vcx/libvcx/Cargo.toml\"))'",
                returnStdout: true
            )
            return version
        }
    }
}

def testPythonWrapper(envn){
    envn.inside {
        sh "python3 -m pytest"
    }

}

def buildRust(envn){
    envn.inside {
        sh "rustc --version"
        sh "gcc --version"
        sh "cd vcx/libvcx; cargo build --features ci --color=never"
    }
}

def testRust(envn) {
    envn.inside {
        sh "cd vcx/libvcx; cargo test --color=never -- --test-threads=1"
    }
}

// Creates a tar.gz file that is installable from pip.
def createPtyhonArtifact(envn, version) {
    dir('vcx/wrappers/python3'){
        sh 'echo Building Python Artifact'
        withEnv(["VCX_VERSION=${version}"]) {
            envn.inside {
                sh(
                    script: "python3 setup.py sdist",
                    returnStdout: true
                )
                archiveArtifacts allowEmptyArchive: true, artifacts: 'dist/**/*.tar.gz'
            }
        }
    }
}

// Update the version and revision in the Cargo.toml file, also the so file .
def updateVersions(app) {
    app.inside {
        sh 'ls -l'
        // TODO fix this ... these *SHOULD* be cargo commands.
        sh 'cd vcx/libvcx; ls -l; ls ../ci/scripts'
        sh 'cd vcx/libvcx; python ../ci/scripts/cargo-update-version'

    }
}

// Updates the libvcx.so file to libvcx<VER>.so
def updateSo(app) {
    app.inside {
        sh 'cd vcx/libvcx; python ../ci/scripts/cargo-update-so'
    }
}

// Creates the debian package for the library, as well as gzips the libvcx.so.<version> file
// Publishes both artifacts.
def createDeb(envn) {
    envn.inside {
        CERT_SCRIPT = 'vcx/ci/scripts/getCert.sh'
        SCRIPT = 'vcx/ci/scripts/gzip_so_file.py'
        FILES = 'vcx/libvcx/target/debug/libvcx.so.* vcx/libvcx/scripts/provision_agent_keys.py'
        DEST = 'libvcx.tar.gz'

        // get Evernym Certificate onto machine
        sh "${CERT_SCRIPT}"

        // build debian
        sh 'cd vcx/libvcx/; cargo deb --no-build'


        if (env.BRANCH_NAME == "master") {
            // archive debian to Jenkins
            archiveArtifacts allowEmptyARchive: true, artifacts: "vcx/libvcx/target/debian/libvcx_*_amd64.deb"

            // archive .so file to Jenkins
            sh "python ${SCRIPT} ${FILES} ${DEST}"
            archiveArtifacts allowEmptyARchive: true, artifacts: DEST

            // upload debian to Repo
            withCredentials([usernameColonPassword(credentialsId: 'jenkins-kraken-svc', variable: 'KRAKEN_CREDENTIALS')]) {
                sh "find vcx/libvcx/target/debian/ -type f -name 'libvcx_*_amd64.deb' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/agency_dev/upload -F file=@{} \\;"
                sh "find vcx/libvcx/target/debian/ -type f -name 'libvcx_*_amd64.deb' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/portal_dev/upload -F file=@{} \\;"
            }
        }
    }
}

// Creates the tgz file that can be 'npm install'-ed.
def testNodeWrapper(envn) {
    dir('vcx/wrappers/node'){
        envn.inside() {
            sh 'npm -v'
            sh 'npm ci'
            sh 'npm run lint'
            sh 'npm test'
            sh 'npm run compile'
            sh 'npm pack'
            archiveArtifacts allowEmptyArchive: true, artifacts: '**/*.tgz'
        }
    }
}

def renameAndUploadNpmTgz(envn, destDir) {
    envn.inside {
        sh "pwd"
        sh "ls"
        CERT_SCRIPT = 'vcx/ci/scripts/getCert.sh'
        sh "${CERT_SCRIPT}"
        filename = 'vcx/wrappers/node/vcx-*.tgz'
        sh "cp ${filename} vcx"
        sh "ls vcx"
        sh "echo ${destDir}"

        withCredentials([usernameColonPassword(credentialsId: 'jenkins-kraken-svc', variable: 'KRAKEN_CREDENTIALS')]) {
            sh "pwd"
            sh "cd vcx; rename \"s/\\.tgz\$/_amd64\\.tgz/\" *.tgz"
            sh "cd vcx; rename \"s/vcx-/vcx_/\" *.tgz"
            sh 'cd vcx; ls -a'
            sh "cd vcx; find . -type f -name '*.tgz' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/npm/upload -F 'file=@{}' \\;"
        }
    }
}

// Creates debian that can install through npm.
def createNpmDeb(app, npmFilename){
    dir('vcx') {
        app.inside {
            CERT_SCRIPT = 'ci/scripts/getCert.sh'
            sh "${CERT_SCRIPT}"
            sh 'ls'
            sh 'python ci/scripts/create_npm_deb.py wrappers/node/vcx-*.tgz'
            sh 'ls'
            archiveArtifacts allowEmptyArchive: true, artifacts: 'vcx*.deb'

            withCredentials([usernameColonPassword(credentialsId: 'jenkins-kraken-svc', variable: 'KRAKEN_CREDENTIALS')]) {
                // upload npm module
                sh "find wrappers/node/ -type f -name 'vcx-*.tgz' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/agency_dev/upload -F file=@{} \\;"
                sh "find wrappers/node/ -type f -name 'vcx-*.tgz' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/portal_dev/upload -F file=@{} \\;"

                // upload debian
                sh "find . -type f -name 'vcx_*_amd64.deb' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/agency_dev/upload -F file=@{} \\;"
                sh "find . -type f -name 'vcx_*_amd64.deb' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST https://kraken.corp.evernym.com/repo/portal_dev/upload -F file=@{} \\;"
            }
        }
    }
}

def fail() {
    def message = [
        message: "$JOB_NAME - Build # $BUILD_NUMBER - fail: Check console output at $BUILD_URL to view the results."
    ]
    slackSend message
}

def buildAndroid(envn) {
    envn.inside {
        ANDROID_SCRIPT_PATH = 'vcx/ci/scripts/androidBuild.sh'
        sh "./${ANDROID_SCRIPT_PATH} arm"
        sh "./${ANDROID_SCRIPT_PATH} x86"
        sh "./${ANDROID_SCRIPT_PATH} arm64"
        //Todo: get parallel processing to work. Currently it fails on Jenkins. It must share files or something
        // parallel([
        //     'arm': { sh "./${ANDROID_SCRIPT_PATH} arm"},
        //     'x86': { sh "./${ANDROID_SCRIPT_PATH} x86"},
        //     'arm64': { sh "./${ANDROID_SCRIPT_PATH} arm64"}
        // ])
    }
}

def packageAndroid(envn) {
    envn.inside {
        ANDROID_SCRIPT_PATH = 'vcx/ci/scripts/androidPackage.sh'
        sh "chmod +x ${ANDROID_SCRIPT_PATH}"
        sh "./${ANDROID_SCRIPT_PATH}"
    }
}

def publishAndroid(envn) {
    envn.inside {
        CERT_SCRIPT = 'vcx/ci/scripts/getCert.sh'
        sh "${CERT_SCRIPT}"
        withCredentials([usernameColonPassword(credentialsId: 'jenkins-kraken-svc', variable: 'KRAKEN_CREDENTIALS')]) {
            sh "echo before publishing"
            sh "find vcx/wrappers/java/vcx/build/outputs/aar/ -type f -name 'com.evernym-vcx_*-release.aar' -exec curl --cacert /tmp/cert/ca.crt -u \"${KRAKEN_CREDENTIALS}\" -X POST  https://kraken.corp.evernym.com/repo/android/upload -F 'file=@{}' \\;"
        }
    }
}
