.PHONY: all
all: payload exploit


.PHONY: payload
payload:
	mkdir -p libnss_x
	cd payload && cargo +nightly build --release --lib
	cp payload/target/release/libx.so libnss_x/x.so.2


.PHONY: exploit
exploit:
	cd exploit && cargo build --release
	cp exploit/target/release/exploit rust_exploit


.PHONY: load
load:
	cd loader && cargo build --release
	./loader/target/release/loader

.PHONY: clean
clean:
	rm -rf target libnss_x
