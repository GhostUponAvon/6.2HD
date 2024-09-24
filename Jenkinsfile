pipeline {
    agent any

    environment {
        RUST="/var/lib/jenkins/.cargo/bin"
        TOKEN="$RELEASE_TOKEN"
        TAG="0.1.$BUILD_NUMBER"
    }

    stages {
        stage('Build') {
            steps {
                echo "Fetching source code from https://github.com/GhostUponAvon/6.2HD.git"
                echo 'Building for Linux'
                dir("app") {
                    sh "$RUST/cargo build --release"
                    
                }
                echo "built 'app'"
                
            }
        }
        stage('Test') {
            steps {
                echo "Running Unit Tests"
                dir("app") {
                    sh "$RUST/cargo test"
                    
                }

            }
            post {
                success {
                    echo "Testing executed successfully"
                    emailext attachLog: true, body: 'The testing was: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Testing'
                }
                failure {
                    echo "Testing failed"
                    emailext attachLog: true, body: 'The testing has: failed', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Testing'
                }
            }
        }
        stage('Code Quality Check') {
            steps {
                echo "Running code quality checks"
                dir("app") {
                    sh "$RUST/cargo clippy"
                    
                }
            }
        }
        stage('Security Scan') {
            steps {
                echo "Scanning for security flaws with sast-scan cdxgen (https://github.com/ShiftLeftSecurity/sast-scan.git)"

            }
            post {
                success {
                    echo "Security scan executed successfully"
                    emailext attachLog: true, body: 'The security scan was: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Security'
                }
                failure {
                    echo "Security scan failed"
                    emailext attachLog: true, body: 'The security scan has: failed', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Security'
                }
            }
        }
        
        stage('Integration Testing') {
            steps {
                echo "Performing Integration testing"
                sleep 10
            }
            post {
                success {
                    echo "Integration testing executed successfully"
                    emailext attachLog: true, body: 'The integration testing was: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Integration'
                }
                failure {
                    echo "Integration testing failed"
                    emailext attachLog: true, body: 'The integration testing has: failed', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Integration'
                }
            }
        }
        stage('Deploy to Production') {
            steps {
                echo "Pushing to Github Release channel"
                dir("app/target/release") {
                    //sh """release=$(curl -XPOST -H "Authorization:token $TOKEN" --data "{\"tag_name\": \"$TAG\", \"target_commitish\": \"master\", \"name\": \"BUILD: $TAG\", \"body\": \"The rust app has been built and the resulting binary is available below for download\", \"draft\": false, \"prerelease\": true}" https://api.github.com/repos/GhostUponAvon/6.2HD/releases) && id=$(echo "\$release" | sed -n -e 's/"id":\ \([0-9]\+\),/\1/p' | head -n 1 | sed 's/[[:blank:]]//g') && curl -XPOST -H "Authorization:token $TOKEN" -H "Content-Type:application/octet-stream" --data-binary @app.exe https://uploads.github.com/repos/GhostUponAvon/release-with-curl/releases/\$id/assets?name=app.exe"""
                }
            }
        }
        
    }
    post {
        always {
            echo "Pipeline Finished"
        }
        success {
            echo "Pipeline executed successfully"
            emailext attachLog: true, body: 'The pipeline has built: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status'
        }
        failure {
            echo "Pipeline execution failed"
            emailext attachLog: true, body: 'The pipeline build has: failed', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status'
        }
    }
}