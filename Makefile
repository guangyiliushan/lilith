# 默认目标
all: lilith-kernel/target/x86_64-unknown-none/release/liblilith_kernel.a

# 清理生成的文件
clean:
	rm -f kernel.bin
	cargo clean --manifest-path lilith-kernel/Cargo.toml

# Rust目标
lilith-kernel/target/x86_64-unknown-none/release/liblilith_kernel.a:
	cargo build --release --manifest-path lilith-kernel/Cargo.toml --target x86_64-unknown-none
