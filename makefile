build:
	riscv64-unknown-elf-gcc \
	  -static -nostdlib -mabi=lp64 -march=rv64gc \
		example_apps/hello_world/hello_world.S \
		-o example_apps/hello_world/hello_world \
		-fno-pie -no-pie -Texample_apps/hello_world/hello_world.ld
	cargo osdk build --target-arch riscv64

.PHONY:
run-debug:
	cargo osdk run --target-arch riscv64 --qemu-args="-s -S"

run:
	cargo osdk run --target-arch riscv64

debug:
	riscv64-elf-gdb \
    -ex 'file /Users/dazhi/Workspace/nike-os/target/riscv64gc-unknown-none-elf/debug/nike-os-osdk-bin' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
