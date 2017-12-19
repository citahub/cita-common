pipeline {
  agent any
  stages {
    stage('Clean') {
      steps {
        sh '.ci-scripts/clean'
      }
    }
    stage('Format') {
      steps {
        sh '.ci-scripts/format'
      }
    }
    stage('Clippy') {
      steps {
        sh '.ci-scripts/clippy'
      }
    }
    stage('Compile') {
      steps {
        sh '.ci-scripts/compile'
      }
    }
    stage('Test') {
      steps {
        sh '.ci-scripts/test'
      }
    }
    stage('Bench') {
      steps {
        sh '.ci-scripts/bench'
      }
    }
  }
}
