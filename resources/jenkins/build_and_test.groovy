/*
 * (c) Copyright, Real-Time Innovations, 2023.  All rights reserved.
 * RTI grants Licensee a license to use, modify, compile, and create derivative
 * works of the software solely for use with RTI Connext DDS. Licensee may
 * redistribute copies of the software provided that all such copies are subject
 * to this license. The software is provided "as is", with no warranty of any
 * type, including any warranty for fitness for any purpose. RTI is under no
 * obligation to maintain or support the software. RTI shall not be liable for
 * any incidental or consequential damages arising out of the use or inability
 * to use the software.
 */

CI_CONFIG = [:]

/*
 * This function generates the stages to Build & Test connector using a specific Rust version.
 *
 * @param rustVersion Version of Rust used to generate the build & test stage.
 * @return The generated Build & Test stages
 */
def getBuildAndTestStages(String rustVersion) {
    def dockerImage = docker.build(
        UUID.randomUUID().toString().split('-')[-1],
        "--pull -f resources/docker/Dockerfile --build-arg RUST_VERSION=${rustVersion} ."
    )
    def versionWorkspace = "${env.WORKSPACE}/${rustVersion}"
    def versionCargoHomeEnv = "CARGO_HOME=${versionWorkspace}/.cargo"

    return {
        /* TODO: Review how multi-version affect artifacts (valgrind, etc) */
        stage("Rust ${rustVersion}") {
            dir("${versionWorkspace}") {
                stage("Setup workspace") {
                    echo "[INFO] Building from ${pwd()}..."
                    checkout scm

                    dockerImage.inside() {
                        downloadAndExtract(
                            installDirectory: '.',
                            flavour: 'connectorlibs'
                        )

                        withEnv([versionCargoHomeEnv]) {
                            sh 'mkdir -p ${CARGO_HOME} && cargo fetch'
                            sh 'cargo build --all-targets --offline'
                        }
                    }
                }

                catchError(buildResult: 'FAILURE', stageResult: 'FAILURE') {
                    stage("Check formatting and linting") {
                        def dockerFlags = [
                            '--network none',
                        ].join(' ')

                        dockerImage.inside(dockerFlags) {
                            withEnv([versionCargoHomeEnv]) {
                                sh 'cargo fmt --all -- --check'
                                sh 'cargo clippy --all-features --all-targets --offline -- -D warnings'
                            }
                        }
                    }
                } // catchError

                catchError(buildResult: 'FAILURE', stageResult: 'FAILURE') {
                    stage("Run tests") {
                        def dockerFlags = [
                            '--network none',
                        ].join(' ')
                        def testFlags = [
                            // Add any specific test flags here
                        ].join(' ')
                        def valgrindCommand = [
                            'nextest run',
                            '--profile ci',
                            '--offline',
                            '--failure-output immediate'
                        ].join(' ')
                        def valgrindFlags = [
                            // cargo-valgrind uses --xml=yes and --xml-socket internally
                            '--suppressions=resources/valgrind/suppressions.txt'
                        ].join(' ')
                        def valgrindFlagsEnv = "VALGRINDFLAGS=${valgrindFlags}"

                        dockerImage.inside(dockerFlags) {
                            withEnv([versionCargoHomeEnv, valgrindFlagsEnv]) {
                                // Docstrings
                                sh "cargo test --all-features --doc --offline -- ${testFlags}"
                                // All others
                                sh "cargo valgrind ${valgrindCommand}"
                            }
                        }

                        junit(testResults: 'target/nextest/ci/test-results.xml')
                    }
                } // catchError

                catchError(buildResult: 'FAILURE', stageResult: 'FAILURE') {
                    stage("Run coverage") {
                        def dockerFlags = [
                            '--network none',
                            '--cap-add=SYS_PTRACE',
                            '--security-opt seccomp=unconfined',
                            '--security-opt apparmor=unconfined'
                        ].join(' ')
                        def tarpaulinFlags = [
                            '--engine Llvm',
                            '--ignore-panics',
                            '--color Never',
                            '--offline',
                        ].join(' ')
                        def testFlags = [

                        ].join(' ')

                        dockerImage.inside(dockerFlags) {
                            withEnv([versionCargoHomeEnv]) {
                                sh "cargo tarpaulin ${tarpaulinFlags} -- ${testFlags}"
                            }
                        }

                        recordCoverage tools: [
                            [
                                parser: 'COBERTURA',
                                pattern: 'tarpaulin-report/cobertura.xml'
                            ],
                        ]

                        publishHTML(target: [
                            allowMissing: false,
                            alwaysLinkToLastBuild: true,
                            keepAll: true,
                            reportDir: 'tarpaulin-report/',
                            reportFiles: 'tarpaulin-report.html',
                            reportName: "Code Coverage Report - Rust ${rustVersion}"
                        ])
                    }
                } // catchError
            }
        }
    }
}

/*
 * Get the Node-JS version from the job name if it is defined there. Example of job name:
 * ci/connector-js/rticonnextdds-connector-js_node-20_latest.
 *
 * @return The list of Rust versions defined in the Job Name. An empty list if it is not defined in the job name.
 */
def getRustVersionsFromJobName() {
    def matcher = env.JOB_NAME =~ /.*rust-(.*)\/.*/

    return matcher ? matcher.group(1).split('_') : []
}

pipeline {
    agent {
        node {
            label 'docker'
        }
    }

    triggers {
        // If it is develop, build at least once a day to test newly created libs.
        // If it is another branch, never build based on timer (31 February = Never).
        cron(env.BRANCH_NAME == 'develop' ? 'H H(18-21) * * *' : '* * 31 2 *')
    }

    options {
        disableConcurrentBuilds()
        /*
            To avoid excessive resource usage in server, we limit the number
            of builds to keep in pull requests
        */
        buildDiscarder(
            logRotator(
                artifactDaysToKeepStr: '',
                artifactNumToKeepStr: '',
                daysToKeepStr: '',
                /*
                   For pull requests only keep the last 10 builds, for regular
                   branches keep up to 20 builds.
                */
                numToKeepStr: changeRequest() ? '10' : '20'
            )
        )
        // Set a timeout for the entire pipeline
        timeout(time: 30, unit: 'MINUTES')
    }

    stages {
        stage('Read CI Config') {
            steps {
                script {
                    CI_CONFIG = readYaml(file: "ci_config.yaml")
                }
            }
        }

        stage('Build & Test') {
            failFast false

            steps {
                script {
                    def rustVersions = getRustVersionsFromJobName()

                    // If the rust versions was not predefined in the job name, read them from the config file.
                    if(!rustVersions) {
                        rustVersions = CI_CONFIG["rust_versions"]
                    }

                    def buildAndTestStages = [:]

                    // Generate the Build & Test stages for every selected rust version.
                    rustVersions.each { version ->
                        buildAndTestStages["Rust ${version}"] = getBuildAndTestStages(version)
                    }

                    parallel buildAndTestStages
                }
            }
        }

        // stage('Publish') {
        //     agent {
        //         dockerfile {
        //             // TODO: Confirm if publish_version should be RUST_VERSION or another variable
        //             additionalBuildArgs  "--build-arg RUST_VERSION=${CI_CONFIG['publish_version']}"
        //             dir 'resources/docker'
        //             reuseNode true
        //             label 'docker'
        //         }
        //     }

        //     when {
        //         beforeAgent true
        //         tag pattern: /v\d+\.\d+\.\d+-dev/, comparator: "REGEXP"
        //     }

        //     steps {
        //         script {
        //             def publishDir = "${env.WORKSPACE}/${CI_CONFIG['publish_version']}"

        //             if(!fileExists(publishDir)) {
        //                 error(
        //                     "The Rust version ${CI_CONFIG['publish_version']} was not used to test connector. Please update the \"publish_version\" field in ci_config.yaml"
        //                 )
        //             }

        //             // TODO: Replace npm publish logic with Rust publish logic if applicable
        //             // withCredentials([
        //             //     string(credentialsId: 'npm-registry', variable: 'NPM_REGISTRY'),
        //             //     string(credentialsId: 'npm-token', variable: 'NPM_TOKEN')
        //             // ]) {
        //             //     dir(publishDir) {
        //             //         sh 'echo "//$NPM_REGISTRY:_authToken=${NPM_TOKEN}" > .npmrc'
        //             //         sh './resources/scripts/publish.sh'
        //             //     }
        //             // }
        //         }
        //     }
        // }
    }

    post {
        cleanup {
            cleanWs()
        }
    }
}
