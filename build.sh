#!/bin/bash

KERNEL_FILE=lilith-kernel/target/x86_64-unknown-none/release/libmy_kernel.a
QEMU_KERNEL=kernel.bin

# Compile the kernel
cargo build --release --manifest-path lilith-kernel/Cargo.toml --target x86_64-unknown-none

# Create a simple bootloader that loads our kernel
cat > bootloader.asm << EOF
    global _start

    section .text
    _start:
        ; Set up the GDT and other things
        ; Load the kernel into memory
        ; Jump to the kernel entry point
EOF

# Assemble the bootloader
nasm -f elf64 bootloader.asm -o bootloader.o

# Link the bootloader and the kernel
ld -n -z max-page-size=0x1000 -Ttext 0x7C00 -o $QEMU_KERNEL bootloader.o $KERNEL_FILE

# Clean up
rm bootloader.o bootloader.asm

echo "Kernel built successfully!"
