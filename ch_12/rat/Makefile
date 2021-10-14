DOCKER_IMAGE = bhr_ch12_server:latest

.PHONY: all
all: agent

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


.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo check
