pipeline {
  agent any
  stages {
    stage('verify Cargo installation') {
      steps {
        sh 'cargo --version'
      }
    }

    stage('Build') {
      steps {
        sh 'cargo build'
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

  }
}