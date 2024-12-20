
# Update

sudo apt update

# Install build essentials

sudo apt install build-essential -y

# Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

# Install curl and unzip

sudo apt install curl unzip

# Install Node.js

curl -fsSL https://fnm.vercel.app/install | bash

source ~/.bashrc

fnm use --install-if-missing 22

# Install Bun

npm install -g bun

# Install protobuf

sudo apt install protobuf-compiler

# Install SQLx CLI

cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Install Bun dependencies

bun install