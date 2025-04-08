# Lilith OS

现代Rust操作系统实现

## 功能特性

- 🚧 模块化微内核架构
- 🚧 引导加载器（Multiboot2 规范）
- 🚧 基础串口调试输出
- 🚧 x86_64 异常处理框架
- 🚧 物理内存管理（Buddy分配器）
- 🚧 虚拟内存管理（分页机制）
- 🚧 内核堆内存分配（slab分配器）
- 🚧 硬件中断控制器（APIC）
- 🚧 进程调度器（Round-Robin）
- 🚧 系统调用接口
- 🚧 用户态进程加载器
- 🚧 虚拟文件系统（VFS 抽象层）
- 🚧 基础块设备驱动
- 🚧 EXT2 文件系统支持
- 🚧 用户空间库（libc 基础）
- 🚧 TTY 终端子系统
- 🚧 内核模块化设计
- 🚧 进程管理
- 🚧 内存管理
- 🚧 虚拟文件系统
- 🚧 进程调度器
- 🚧 ACPI电源管理
- 🚧 网络协议栈
- 🚧 GUI图形界面
- 🚧 多格式文件支持
- 🚧 物理内存管理
- 🚧 QEMU集成测试
- 🚧 异步驱动框架
- 🚧 硬件加速支持（GPU/Vulkan）

## 测试

```bash
cargo test -- format_parser
```

## 快速开始

```bash
# 安装依赖
rustup target add x86_64-unknown-none
cargo install xargo

# 完整构建
make build

# QEMU启动测试
make run
```

## 制作启动盘

1. 使用Rufus选择生成的target/lilith.iso
2. 选择"DD镜像"写入模式
3. 目标设备选择U盘
4. 开始写入后等待完成

## 项目目录结构

├── bootloader/        # 启动加载器
├── kernel/            # 内核核心
│   ├── src/
│   │   ├── memory/    # 内存管理
│   │   ├── proc/      # 进程管理
│   │   ├── vfs/       # 虚拟文件系统
│   │   └── var/       # 系统日志
├── driver/            # 设备驱动
└── scripts/

## lilith文件结构

/
├── bin/        # 基础命令二进制文件
├── boot/       # 系统引导文件（内核、GRUB）
├── dev/        # 设备文件接口（磁盘、终端等）
├── etc/        # 系统配置文件（passwd、ssh等）
├── home/       # 普通用户主目录
├── lib/        # 核心共享库（.so文件）
├── lost+found/ # 系统崩溃恢复文件
├── media/      # 自动挂载移动设备
├── mnt/        # 临时手动挂载点
├── opt/        # 第三方软件安装目录（如Chrome）
├── proc/       # 进程/内核信息的虚拟文件系统
├── root/       # root用户专属目录
├── run/        # 运行时进程数据（PID等）
├── sbin/       # 系统管理命令（fdisk、shutdown）
├── srv/        # 服务数据（网站、FTP）
├── sys/        # 内核设备树信息
├── tmp/        # 临时文件（重启清空）
├── usr/
│   ├── bin/    # 用户级应用程序
│   ├── sbin/   # 管理员高级工具
│   └── src/    # 内核源代码
└── var/
    ├── log/    # 系统日志
    └── www/    # Web服务数据
