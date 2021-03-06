target := riscv64imac-unknown-none-elf
mode := debug
kernel := target/$(target)/$(mode)/os
bin := target/$(target)/$(mode)/kernel.bin

objdump := rust-objdump --arch-name=riscv64
objcopy := rust-objcopy --binary-architecture=riscv64

.PHONY: kernel build clean qemu run env

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview rustfmt
	rustup target add $(target)

kernel:
	cargo build

$(bin): kernel
	$(objcopy) $(kernel) --strip-all -O binary $@

asm: kernel
	$(objdump) --disassemble $(kernel) | less

elf: kernel
	$(objdump) --all-headers $(kernel) | less

build: $(bin)

clean:
	cargo clean
	rm -rf *.dtb *.dts

qemu: build
	qemu-system-riscv64 \
		-machine virt   \
		-nographic      \
		-bios default   \
		-device loader,file=$(bin),addr=0x80200000


qemu-gdb: build
	qemu-system-riscv64 \
		-gdb tcp::9000  \
		-machine virt   \
		-nographic      \
		-bios default   \
		-S              \
		-device loader,file=$(bin),addr=0x80200000

run: build qemu

dtc:
	qemu-system-riscv64 -machine virt -machine dumpdtb=riscv64-virt.dtb -bios default
	dtc -I dtb -O dts -o riscv64-virt.dts riscv64-virt.dtb
