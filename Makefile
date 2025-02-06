ADDRESS := "0.0.0.0"
DOMAIN := "example.com"
POSTGRES_PASSWORD := $(shell echo -n SYSTEM_PASSWORD | sha256sum | awk '{print $$1}')

generate-env:
	@{ \
	echo "APP_ADDRESS=$(ADDRESS)"; \
	echo "APP_PORT=8080"; \
	echo "PUBLIC_MANAGEMENT_DOMAIN=management.$(DOMAIN)"; \
	echo "PUBLIC_MARKETING_DOMAIN=www.$(DOMAIN)"; \
	echo "PUBLIC_SHIPPING_DOMAIN=shipping.$(DOMAIN)"; \
	echo "JWT_SECRET=$(shell openssl rand -base64 32)"; \
	echo "POSTGRES_USER=$(POSTGRES_USER)"; \
	echo "POSTGRES_PASSWORD=$(POSTGRES_PASSWORD)"; \
	echo "POSTGRES_DB=$(POSTGRES_DB)"; \
	echo "POSTGRES_HOST=$(POSTGRES_HOST)"; \
	echo "POSTGRES_PORT=$(POSTGRES_PORT)"; \
	echo "PG_DATA=/var/lib/postgresql/data"; \
	echo "DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)"; \
	} > .env

# install all ubuntu dependencies
install/ubuntu:
	sudo apt update
	sudo apt install -y build-essential
	sudo apt install curl unzip
	sudo apt install protobuf-compiler

install/node:
	curl -o- https://fnm.vercel.app/install | bash
	. ~/.bashrc
	fnm use --install-if-missing 22

install/bun:
	curl -o- https://bun.sh/install | bash
	
install/rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
	cargo install sqlx-cli --no-default-features --features postgres
	
install/go:
	go install github.com/a-h/templ/cmd/templ@latest
	go mod tidy

install: 
	make install/ubuntu
	make generate-env
	make -j install/bun install/node install/rust install/go postgres

postgres:
	docker compose down -v && docker compose up -d

migrate:
	sqlx migrate run

lint:
	cargo clippy

build/go:
	templ generate
	go build -o ./target/golang/frontend ./cmd/main.go

# move htmx files from node_modules to the build directory
# move alpine.js files from node_modules to the build directory
# move tailwindcss output to the build directory
build/assets:
	bun run build.ts 

build/rust:
	cargo build

build: 
	make -j3 build/go build/typescript build/rust

test:
	cargo test

local-ci:
	make install
	make test
	make lint

dev/go:
	go run ./src/views/main.go

dev/tailwind:
	bun tailwindcss -i src/views/components/app.css -o src/views/assets/out.css --watch

dev/typescript:
	bun run --hot build.ts

dev/templ:
	templ generate --watch --proxybind="0.0.0.0" --proxy="http://localhost:8080" --open-browser=false	--cmd="make dev/go"

dev:
	make -j4 dev/go dev/tailwind dev/typescript dev/templ