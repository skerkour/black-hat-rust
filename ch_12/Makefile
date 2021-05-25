DOCKER_IMAGE = bhr_ch12_server:latest
AGENT_BUILDER_DOCKER_IMAGE = bhr_ch12_agent_builder:latest

.PHONY: all
all: agent agent docker

.PHONY: build
build: agent client server

.PHONY: agent
agent: agent_linux_docker agent_windows_docker


.PHONY: agent_linux_docker
agent_linux_docker:
	docker run -ti --rm -v `pwd`:/bhr $(AGENT_BUILDER_DOCKER_IMAGE) make agent_linux

.PHONY: agent_windows_docker
agent_windows_docker:
	docker run -ti --rm -v `pwd`:/bhr $(AGENT_BUILDER_DOCKER_IMAGE) make agent_windows

.PHONY: agent_linux
agent_linux:
	cargo build -p agent --release --target x86_64-unknown-linux-musl
	strip -s target/x86_64-unknown-linux-musl/release/agent
	upx -9 target/x86_64-unknown-linux-musl/release/agent

.PHONY: agent_windows
agent_windows:
	cargo build -p agent --release --target x86_64-pc-windows-gnu
	strip -s target/x86_64-pc-windows-gnu/release/agent.exe
	upx -9 --force target/x86_64-pc-windows-gnu/release/agent.exe

.PHONY: client
client:
	cargo build -p client --release

.PHONY: server
server:
	cargo build -p server --release

.PHONY: docker
docker:
	docker build -t $(DOCKER_IMAGE) .


.PHONY: docker_agent
docker_agent:
	docker build -t $(AGENT_BUILDER_DOCKER_IMAGE) . -f Dockerfile.agent


.PHONY: dev
dev:
	cargo watch -x 'run -p server'

.PHONY: fmt
fmt:
	cargo fmt
