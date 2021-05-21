DOCKER_IMAGE = bhr_ch12_server:latest

.PHONY: all
all: build docker

.PHONY: build
build: agent client server

.PHONY: agent
agent:
	cargo build -p agent --release --target x86_64-unknown-linux-musl
	strip -s target/x86_64-unknown-linux-musl/release/agent
	upx -9 target/x86_64-unknown-linux-musl/release/agent

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
