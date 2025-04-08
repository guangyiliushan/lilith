KERNEL := target/x86_64-unknown-none/release/kernel
ISO := target/lilith.iso

build:
    @cargo build --workspace --release

iso: build
    @mkdir -p isofiles/boot/grub
    @cp $(KERNEL) isofiles/boot/kernel.bin
    @grub-mkrescue -o $(ISO) isofiles

run: iso
    @qemu-system-x86_64 -cdrom $(ISO)

# 添加测试目标
test: iso
    qemu-system-x86_64 -cdrom $(ISO) -serial stdio -d cpu_reset

# 添加文档生成
doc:
    cargo doc --workspace --no-deps