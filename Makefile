
ADDRESS := "0.0.0.0"
DOMAIN := "example.com"
POSTGRES_PASSWORD := $(shell echo -n SYSTEM_PASSWORD | sha256sum | awk '{print $$1}')

generate-env:
	@echo "APP_ADDRESS=$(ADDRESS)" > .env
	@echo "APP_PORT=8080" >> .env
	@echo "PUBLIC_MANAGEMENT_DOMAIN=management.$(DOMAIN)" >> .env
	@echo "PUBLIC_MARKETING_DOMAIN=www.$(DOMAIN)" >> .env
	@echo "PUBLIC_SHIPPING_DOMAIN=shipping.$(DOMAIN)" >> .env
	@echo "JWT_SECRET=$(shell openssl rand -base64 32)" >> .env
	@echo "POSTGRES_USER=$(POSTGRES_USER)" >> .env
	@echo "POSTGRES_PASSWORD=$(POSTGRES_PASSWORD)" >> .env
	@echo "POSTGRES_DB=$(POSTGRES_DB)" >> .env
	@echo "POSTGRES_HOST=$(POSTGRES_HOST)" >> .env
	@echo "POSTGRES_PORT=$(POSTGRES_PORT)" >> .env
	@echo "PG_DATA=/var/lib/postgresql/data" >> .env
	@echo "DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)" >> .env

install:
	chmod +x ./install.sh && ./install.sh
	make generate-env
	bun install
	go mod tidy
	go install github.com/a-h/templ/cmd/templ@latest
	cargo install sqlx-cli --no-default-features --features native-tls,postgres	
	make postgres
	make migrate
	cargo build

postgres:
	docker compose down -v && docker compose up -d && sleep 10

migrate:
	sqlx migrate run && cargo sqlx prepare

lint:
	cargo clippy

build/go:
	go build -o ./target/golang/frontend ./src/views/main.go

build: 
	templ generate
	bun run build.ts
	go build -o ./target/golang/frontend ./src/views/main.go
	cargo build
	
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