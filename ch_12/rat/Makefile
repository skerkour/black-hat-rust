DOCKER_IMAGE = bhr_ch12_server:latest

.PHONY: all
all: agent

.PHONY: agent
agent: agent_linux agent_windows

.PHONY: agent_linux
agent_linux:
	cross build -p agent --release --target x86_64-unknown-linux-musl


.PHONY: agent_windows
agent_windows:
	cross build -p agent --release --target x86_64-pc-windows-gnu


.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo check
