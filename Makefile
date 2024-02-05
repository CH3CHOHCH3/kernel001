.PHONY: bios kernel qemu gdb-server gdb-client clean
.DEFAULT_GOAL := all

all: bios kernel qemu

QEMU = qemu-system-riscv64
GDB = riscv64-unknown-elf-gdb

BIOS = opensbi/build/platform/generic/firmware
RELEASE = target/riscv64gc-unknown-none-elf/release

QEMUOPTS = -machine virt -nographic -bios $(BIOS)/fw_jump.bin -m 128M
QEMUOPTS += -device loader,file=$(RELEASE)/kernel.bin,addr=0x80200000


bios:
	make -C opensbi PLATFORM=generic FW_JUMP_ADDR=0X80200000 \
		PLATFORM_RISCV_XLEN=64 LLVM=1

kernel:
	cargo build --release
	rust-objcopy --strip-all $(RELEASE)/kernel -O binary $(RELEASE)/kernel.bin

qemu: bios kernel
	$(QEMU) $(QEMUOPTS)

gdb-server: bios kernel
	$(QEMU) $(QEMUOPTS) -s -S

gdb-client: bios kernel
	$(GDB) -ex 'file $(RELEASE)/kernel' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'

clean:
	cargo clean
	make -C opensbi clean
