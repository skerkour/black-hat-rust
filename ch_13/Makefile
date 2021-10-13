DOCKER_IMAGE = bhr_ch13_server:latest

.PHONY: all
all: agent client docker_server

.PHONY: build
build: agent client server


.PHONY: agent
agent: agent_linux agent_windows

.PHONY: agent_linux
agent_linux:
	cross build -p agent --release --target x86_64-unknown-linux-musl
	strip -s target/x86_64-unknown-linux-musl/release/agent
	upx -9 target/x86_64-unknown-linux-musl/release/agent

.PHONY: agent_windows
agent_windows:
	cross build -p agent --release --target x86_64-pc-windows-gnu
	strip -s target/x86_64-pc-windows-gnu/release/agent.exe
	upx -9 --force target/x86_64-pc-windows-gnu/release/agent.exe



.PHONY: clean_agent
clean_agent:
	cargo run -p agent -- clean


.PHONY: client
client:
	cargo build -p client --release


.PHONY: server
server:
	cargo build -p server --release


.PHONY: docker_server
docker_server:
	docker build -t $(DOCKER_IMAGE) . -f Dockerfile.server

.PHONY: dev
dev:
	cargo watch -x 'run -p server'

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo check
