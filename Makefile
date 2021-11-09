MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

run-db:
	docker-compose up

run-app:
	cargo run

build:
	cargo build

migration:
	diesel migration run

clean:
	cargo clean
	docker-compose down --rmi all