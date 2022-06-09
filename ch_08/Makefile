.PHONY: execute
execute:
	cd executor && cargo run


.PHONY: dump_shellcode
dump_shellcode:
	objdump -D -b binary -mi386 -Mx86-64 -Mintel -z shellcode.bin


.PHONY: fmt
fmt:
	cd hello_world && cargo fmt
	cd executor && cargo fmt



.PHONY: run_hello_world
run_hello_world: hello_world execute

.PHONY: hello_world
hello_world:
	cd hello_world && cargo build --release
	strip -s hello_world/target/release/hello_world
	objcopy -O binary hello_world/target/release/hello_world shellcode.bin

.PHONY: dump_hello_world
dump_hello_world: hello_world dump_shellcode
# objdump -Mintel -s -d hello_world/target/release/hello_world



.PHONY: shell
shell:
	cd shell && cargo build --release
	strip -s shell/target/release/shell
	objcopy -O binary shell/target/release/shell shellcode.bin

.PHONY: dump_shell
dump_shell: shell dump_shellcode

.PHONY: run_shell
run_shell: shell execute

.PHONY: build_shell_debug
build_shell_debug:
	cd shell && cargo build


.PHONY: tcp
tcp:
	cd reverse_tcp && cargo build --release
	strip -s reverse_tcp/target/release/reverse_tcp
	objcopy -O binary reverse_tcp/target/release/reverse_tcp shellcode.bin

.PHONY: dump_tcp
dump_tcp: tcp dump_shellcode

.PHONY: run_tcp
run_tcp: tcp execute
