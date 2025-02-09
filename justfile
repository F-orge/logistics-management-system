
format:
  cargo fmt

lint:
  cargo clippy

build: postgres format lint
  cargo build

build-release: postgres format lint
  cargo build --release

test: build 
  cargo test --workspace

postgres:
  docker compose up -d postgres

tailwind-watch:
  npx tailwindcss -i assets/app.css -o dist/css/out.css --watch

pre-release: build-release
  