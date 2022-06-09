.PHONY: bundle
bundle: x86_64 aarch64
	rm -rf bundle.zip
	zip -j bundle.zip target/agent.linux_x86_64 target/agent.linux_aarch64


.PHONY: x86_64
x86_64:
	cross build -p agent --release --target x86_64-unknown-linux-musl
# strip -s target/x86_64-unknown-linux-musl/release/agent
#	upx -9 target/x86_64-unknown-linux-musl/release/agent
	mv target/x86_64-unknown-linux-musl/release/agent target/agent.linux_x86_64


.PHONY: aarch64
aarch64:
	cross build -p agent --release --target aarch64-unknown-linux-musl
# strip -s target/aarch64-unknown-linux-musl/release/agent
	upx -9 target/aarch64-unknown-linux-musl/release/agent
	mv target/aarch64-unknown-linux-musl/release/agent target/agent.linux_aarch64

.PHONY: check
check:
	cargo check
