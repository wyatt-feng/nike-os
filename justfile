build:
  cargo osdk build --target-arch riscv64

run:
  cargo osdk run --target-arch riscv64

debug:
  riscv64-elf-gdb \
    -ex 'file /Users/dazhi/Workspace/nike-os/target/riscv64gc-unknown-none-elf/debug/nike-os-osdk-bin' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
