DOCKER_IMAGE = bhr_ch10_server:latest

.PHONY: all
all: build docker

.PHONY: build
build: agent client server

.PHONY: agent
agent:
	cargo build -p agent --release

.PHONY: client
client:
	cargo build -p client --release

.PHONY: server
server:
	cargo build -p server --release

.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE) .

.PHONY: dev
dev:
	cargo watch -x 'run -p server'

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo check
