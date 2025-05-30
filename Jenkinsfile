pipeline {
   agent any

   stages {
      stage('Clean') {
         steps {
            sh 'cargo clean'
         }
      }

      stage('Test') {
         environment {
            RUST_BACKTRACE = 1
         }
         steps {
            sh 'cargo test'
         }
      }

      stage('Wasm Build') {
         steps {
            dir('maze_wasm') {
               withEnv(['RUSTFLAGS=--cfg getrandom_backend="wasm_js"']) {
                  sh 'cargo build --release --target wasm32-unknown-unknown'
               }
               sh 'wasm-bindgen --target web ../target/wasm32-unknown-unknown/release/maze_wasm.wasm --out-dir ./pkg'
            }
         }
      }

      stage('Deploy') {
         when {
            expression { env.BRANCH_NAME == "master" }
         }
         steps {
            dir('publish') {
               dir('pkg') {
                  sh 'cp ../../maze_wasm/pkg/maze_wasm.js .'
                  sh 'cp ../../maze_wasm/pkg/maze_wasm_bg.wasm .'
               }
               sh 'cp ../maze_wasm/index.html .'
               sh 'cp ../maze_wasm/index.js .'
               sh 'cp ../maze_wasm/stylesheet.css .'
               sshagent (credentials: ['jenkins-ssh-nfs']) {
                  sh 'rsync -avr -e "ssh -l flandoo_brickcodes -o StrictHostKeyChecking=no" --exclude ".git" --exclude "pkg@tmp" . ssh.nyc1.nearlyfreespeech.net:/home/public/mazes'
               }
            }
         }
      }
   }
}
