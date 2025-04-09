# 定义编译器和编译选项
-CC = gcc
-CFLAGS = -Wall -Werror

-# 定义目标文件和源文件
-TARGET = lilith-kernel
-SRCS = $(wildcard *.c)
-OBJS = $(SRCS:.c=.o)

# 默认目标
-all: $(TARGET)
+all: target/x86_64-unknown-none/release/liblilith_kernel.a

-# 链接目标文件生成可执行文件
-$(TARGET): $(OBJS)
-	$(CC) $(CFLAGS) -o $@ $^
-
-# 编译源文件生成目标文件
-%.o: %.c
-	$(CC) $(CFLAGS) -c $< -o $@

# 清理生成的文件
clean:
-	rm -f $(OBJS) $(TARGET)
+	rm -f kernel.bin
	cargo clean

# Rust目标
target/x86_64-unknown-none/release/liblilith_kernel.a:
	cargo build --release --target x86_64-unknown-none

