.PHONY: build run

build:
		cargo osdk build --target-arch riscv64

run:
		cargo osdk run --target-arch riscv64
