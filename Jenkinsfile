pipeline {
  agent any

  post {
    failure {
      updateGitlabCommitStatus name: 'CI Test', state: 'failed'
    }

    success {
      updateGitlabCommitStatus name: 'CI Test', state: 'success'
    }
  }

  stages {

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

    stage('Build') {
      steps {
        sh '.ci-scripts/build'
      }
    }

    stage('Unit test (sha3 & secp256k1)') {
      environment {
        HASH_ALGO = 'sha3hash'
        CRYPTO_ALGO = 'secp256k1'
      }
      steps {
        sh '.ci-scripts/test ${HASH_ALGO} ${CRYPTO_ALGO}'
      }
    }

    stage('Unit test (blake2b & ed25519)') {
      environment {
        HASH_ALGO = 'blake2bhash'
        CRYPTO_ALGO = 'ed25519'
      }
      steps {
        sh '.ci-scripts/test ${HASH_ALGO} ${CRYPTO_ALGO}'
      }
    }

    stage('Unit test in (sm3 & sm2)') {
      environment {
        HASH_ALGO = 'sm3hash'
        CRYPTO_ALGO = 'sm2'
      }
      steps {
        sh '.ci-scripts/test ${HASH_ALGO} ${CRYPTO_ALGO}'
      }
    }

    stage('Bench') {
      steps {
        sh '.ci-scripts/bench'
      }
    }
  }
}
