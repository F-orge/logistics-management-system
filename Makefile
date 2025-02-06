#!/bin/bash

ADDRESS := "0.0.0.0"
DOMAIN := "example.com"
POSTGRES_PASSWORD := $(shell [ -z "${SYSTEM_PASSWORD}" ] && echo "randompassword" || echo -n ${SYSTEM_PASSWORD} | sha256sum | awk '{print $$1}' )
POSTGRES_USER := ${shell [ -z "${POSTGRES_USER}" ] && echo "postgres" || echo ${POSTGRES_USER}}
POSTGRES_DB := ${shell [ -z "${POSTGRES_DB}" ] && echo "postgres" || echo ${POSTGRES_DB}}
POSTGRES_HOST := ${shell [ -z "${POSTGRES_HOST}" ] && echo "localhost" || echo ${POSTGRES_HOST}}
POSTGRES_PORT := ${shell [ -z "${POSTGRES_PORT}" ] && echo "5432" || echo ${POSTGRES_PORT}}
STORAGE_DIRECTORY_URL := ${shell [ -z "${STORAGE_DIRECTORY_URL}" ] && echo "./storage" || echo ${STORAGE_DIRECTORY_URL}}

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
	echo "STORAGE_DIRECTORY_URL=$(STORAGE_DIRECTORY_URL)"; \
	echo "DATABASE_URL=postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)"; \
	} > .env

unzip := $(shell command -v unzip > /dev/null && echo true || echo false)
protoc := $(shell command -v protoc > /dev/null && echo true || echo false)

install/ubuntu:
	sudo apt update
	sudo apt install -y build-essential
ifeq ($(unzip),false)
	sudo apt install curl unzip
endif
ifeq ($(protoc),false)
	sudo apt install protobuf-compiler
endif


node := $(shell command -v node > /dev/null && echo true || echo false)
install/node:
ifeq ($(node),false)
	curl -o- https://fnm.vercel.app/install | bash
	. ~/.bashrc
	fnm install 22
endif
	npm install
	
rust := $(shell command -v rustc > /dev/null && echo true || echo false)
sqlx := $(shell command -v sqlx > /dev/null && echo true || echo false)
install/rust:
ifeq ($(rust),false)
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
endif
ifeq ($(sqlx),false)
	cargo install sqlx-cli --no-default-features --features postgres
endif
	
go := $(shell command -v go > /dev/null && echo true || echo false)
install/go:
ifeq ($(go),false)
	snap install go --classic
endif
	go install github.com/a-h/templ/cmd/templ@latest
	go get -u golang.org/x/lint/golint
	go mod tidy

install: 
	make generate-env
	make install/ubuntu
	make -j install/node install/rust install/go
	make postgres
	sleep 10
	make migrate

postgres:
	docker compose down -v && docker compose up -d

migrate:
	sqlx migrate run

lint/rust:
	cargo clippy --no-deps

lint/go:
	golangci-lint run

lint:
	make -j lint/rust lint/go

build/go:
	templ generate
	go build -o ./target/golang/frontend ./cmd/main.go

build/assets:
	cp ./node_modules/htmx.org/dist/htmx.min.js ./dist/js/htmx.min.js
	cp ./node_modules/alpinejs/dist/cdn.min.js ./dist/js/alpine.min.js
	make build/tailwindcss

build/tailwindcss:
	npx tailwindcss -i web/app.css -o dist/css/out.css --minify

build/rust:
	cargo build

build: 
	make -j3 build/go build/assets build/rust

dev/tailwind:
	npx tailwindcss -i web/app.css -o dist/css/out.css --watch

dev/go:
	templ generate --watch --proxybind="0.0.0.0" --proxy="http://localhost:8080" --open-browser=false	--cmd="go run ./cmd/main.go"

dev/rust:
	cargo watch -x run

dev:
	make -j dev/go dev/tailwind dev/go dev/rust

test:
	cargo test --workspace

local-ci:
	make -j test lint
	make build