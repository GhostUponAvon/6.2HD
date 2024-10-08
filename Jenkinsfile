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
                    emailext attachLog: true, body: 'The testing executed: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Testing'
                }
                failure {
                    echo "Testing failed"
                    emailext attachLog: true, body: 'The testing executed: unsuccessfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Testing'
                }
            }
        }
        stage('Code Quality Check') {
            steps {
                echo "Running code quality and security checks"
                dir("app") {
                    sh "$RUST/cargo clippy"
                    
                }
            }
        }
        stage('Deploy to Production') {
            steps {
                echo "Pushing to Github Release channel"
                dir("app/target/release") {
                    sh '''release=$(curl -XPOST -H "Authorization:token $TOKEN" --data "{\\"tag_name\\": \\"$TAG\\", \\"target_commitish\\": \\"master\\", \\"name\\": \\"BUILD: $TAG\\", \\"body\\": \\"The rust app has been built and the resulting binary is available below for download\\", \\"draft\\": false, \\"prerelease\\": true}" https://api.github.com/repos/GhostUponAvon/6.2HD/releases) && id=$(echo "\$release" | sed -n -e 's/"id":\\ \\([0-9]\\+\\),/\\1/p' | head -n 1 | sed 's/[[:blank:]]//g') && curl -XPOST -H "Authorization:token $TOKEN" -H "Content-Type:application/octet-stream" --data-binary @app https://uploads.github.com/repos/GhostUponAvon/6.2HD/releases/\$id/assets?name=app'''
                }
            }
        }
        
    

    stage('Monitoring') {
            steps {
                echo "Performing Monitoring"
                sh '''echo $(./app/target/release/app 34 56) > /var/lib/jenkins/workspace/logs/6.2HD-build-\$TAG.txt'''
            }
            post {
                success {
                    echo "Monitoring executed successfully"
                    emailext attachLog: true, body: 'Monitoring executed: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Monitoring'
                }
                failure {
                    echo "Monitoring indicates a runtime error"
                    emailext attachLog: true, body: 'Monitoring executed: unsuccessfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status: Monitoring'
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
            emailext attachLog: true, body: 'The pipeline has completed: successfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status'
        }
        failure {
            echo "Pipeline execution failed"
            emailext attachLog: true, body: 'The pipeline has completed: unsuccessfully', to:'mikehodgetheboss@gmail.com', subject: 'Pipeline build status'
        }
    }
}