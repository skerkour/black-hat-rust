.PHONY: execute
execute:
	cd executor && cargo run


.PHONY: dump_shellcode
dump_shellcode:
	objdump -D -b binary -mi386 -Mx86-64 -Mintel shellcode.bin


.PHONY: fmt
fmt:
	cd hello_world && cargo fmt
	cd executor && cargo fmt


.PHONY: hello_world
hello_world:
	cd hello_world && cargo +nightly build --release
	strip -s hello_world/target/release/hello_world
	objcopy -O binary hello_world/target/release/hello_world shellcode.bin


.PHONY: dump_hello_world
dump_hello_world:
	objdump -Mintel -s -d hello_world/target/release/hello_world
