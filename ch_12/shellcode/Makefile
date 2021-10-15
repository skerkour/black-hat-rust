AARCH64_DOCKER_IMAGE = black_hat_rust/ch12_aarch64
x86_64_DOCKER_IMAGE = black_hat_rust/ch12_x86_64
ARMV7_DOCKER_IMAGE = black_hat_rust/ch12_armv7


.PHONY: docker
docker: docker_image_aarch64 docker_image_x86_64 docker_image_armv7

.PHONY: docker_image_aarch64
docker_image_aarch64:
	docker build -t $(AARCH64_DOCKER_IMAGE) . -f Dockerfile.aarch64

.PHONY: docker_image_x86_64
docker_image_x86_64:
	docker build -t $(x86_64_DOCKER_IMAGE) . -f Dockerfile.x86_64

.PHONY: docker_image_armv7
docker_image_armv7:
	docker build -t $(ARMV7_DOCKER_IMAGE) . -f Dockerfile.armv7



.PHONY: execute_aarch64
execute_aarch64:
	cd executor && cross run --target aarch64-unknown-linux-gnu

.PHONY: execute_x86_64
execute_x86_64:
	cd executor && cross run --target x86_64-unknown-linux-gnu

.PHONY: execute_armv7
execute_armv7:
	cd executor && cross run --target armv7-unknown-linux-gnueabihf


# .PHONY: dump_shellcode
# dump_shellcode:
# 	objdump -D -b binary -mi386 -Mx86-64 -Mintel -z shellcode.bin


.PHONY: fmt
fmt:
	cd hello_world && cargo fmt
	cd executor && cargo fmt

.PHONY: hello_world_aarch64
hello_world_aarch64:
	docker run --rm -ti -v `pwd`:/app $(AARCH64_DOCKER_IMAGE)

.PHONY: hello_world_aarch64_docker
hello_world_aarch64_docker:
	cd hello_world && cargo +nightly build --release --target aarch64-unknown-linux-gnu
	aarch64-linux-gnu-strip -s hello_world/target/aarch64-unknown-linux-gnu/release/hello_world
	aarch64-linux-gnu-objcopy -O binary hello_world/target/aarch64-unknown-linux-gnu/release/hello_world shellcode.bin
	cp shellcode.bin executor/


.PHONY: hello_world_x86_64
hello_world_x86_64:
	docker run --rm -ti -v `pwd`:/app $(x86_64_DOCKER_IMAGE)

.PHONY: hello_world_x86_64_docker
hello_world_x86_64_docker:
	cd hello_world && cargo +nightly build --release --target x86_64-unknown-linux-gnu
	strip -s hello_world/target/x86_64-unknown-linux-gnu/release/hello_world
	objcopy -O binary hello_world/target/x86_64-unknown-linux-gnu/release/hello_world shellcode.bin
	cp shellcode.bin executor/


.PHONY: hello_world_armv7
hello_world_armv7:
	docker run --rm -ti -v `pwd`:/app $(ARMV7_DOCKER_IMAGE)

.PHONY: hello_world_armv7_docker
hello_world_armv7_docker:
	cd hello_world && cargo +nightly build --release --target armv7-unknown-linux-gnueabihf
	arm-linux-gnueabihf-strip -s hello_world/target/armv7-unknown-linux-gnueabihf/release/hello_world
	arm-linux-gnueabihf-objcopy -O binary hello_world/target/armv7-unknown-linux-gnueabihf/release/hello_world shellcode.bin
	cp shellcode.bin executor/


.PHONY: dump_hello_world_aarch64
dump_hello_world_aarch64:
	docker run --rm -ti -v `pwd`:/app $(AARCH64_DOCKER_IMAGE) aarch64-linux-gnu-objdump -D -b binary -maarch64 shellcode.bin


.PHONY: dump_hello_world_x86_64
dump_hello_world_x86_64:
	docker run --rm -ti -v `pwd`:/app $(x86_64_DOCKER_IMAGE) objdump -D -b binary -mi386 -Mx86-64 -Mintel -z  shellcode.bin


.PHONY: dump_hello_world_armv7
dump_hello_world_armv7:
	docker run --rm -ti -v `pwd`:/app $(ARMV7_DOCKER_IMAGE) arm-linux-gnueabihf-objdump -D -b binary -marm  shellcode.bin


.PHONY: clean
clean:
	rm -rf hello_world/target/ executor/target/ shellcode.bin
	docker rmi $(x86_64_DOCKER_IMAGE) $(AARCH64_DOCKER_IMAGE)
