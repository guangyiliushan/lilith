# Lilith OS

该项目旨在通过学习Rust语言和操作系统原理，实现一个基本的操作系统。

- lilith OS是一个基于Rust语言的操作系统，旨在提供一个简单、高效、兼容性良好的操作系统环境。
- lilith-kernel 是一个基于Rust语言的操作系统内核，旨在提供一个简单、高效的操作系统环境。

## 代码要求

- **低嵌套**：
  - 每个函数或方法的嵌套层级不应超过 3 层。若嵌套层级可能超过 3 层，应将部分逻辑提取为独立的函数或方法。
  - 避免在循环中嵌套过多的条件判断，可通过提前返回或使用卫语句简化逻辑。
  - 使用面向对象的编程风格，将复杂的逻辑封装为类或模块，减少嵌套层级。
  - 避免使用过于复杂的表达式和语句，尽量使用简洁的代码结构。
  - 避免使用过多的条件判断和循环，应使用更高效的算法和数据结构。
  - 避免使用过于长的函数或方法，应将功能拆分成多个小的函数或方法。
- **逻辑清晰**：
  - 每个函数或方法应只负责单一的功能，遵循单一职责原则。
  - 函数和变量的命名应具有描述性，准确反映其功能或用途。
  - 代码逻辑应按照合理的顺序编写，避免出现跳跃或混乱的逻辑。
- **代码风格统一**：
  - 遵循 Rust 社区的代码风格指南，使用 snake_case 命名变量和函数，CamelCase 命名类型。
  - 统一使用制表符进行缩进，避免混用。
  - 代码中的注释、空格和标点符号的使用应保持一致。
- **代码可读性**：
  - 避免使用过于复杂的表达式和语句，尽量将复杂逻辑拆分成多个步骤。
  - 合理使用空行和注释来分隔不同的代码块，提高代码的可视性。
  - 避免使用魔术数字和硬编码的字符串，应使用常量或枚举来代替。
  - 变量和函数的命名应具有描述性，准确反映其功能或用途。
  - 在复杂的代码逻辑或关键步骤处添加行注释，解释代码的意图和实现方式。
  - 避免使用无意义或重复的注释，注释应简洁明了。

## 功能特性

- 🚧 基础内核功能
  - [ ] 内核启动
  - [ ] 内核初始化
  - [ ] 中断处理
  - [ ] 异常处理
- 🚧 模块化微内核架构
  - [ ] 核心服务分离（进程管理、内存管理、进程间通信）
    - [ ] 设计进程管理核心服务的接口和架构
    - [ ] 实现进程管理核心服务的基本功能
    - [ ] 对进程管理核心服务进行单元测试
    - [ ] 设计内存管理核心服务的接口和架构
    - [ ] 实现内存管理核心服务的基本功能
    - [ ] 对内存管理核心服务进行单元测试
    - [ ] 设计进程间通信核心服务的接口和架构
    - [ ] 实现进程间通信核心服务的基本功能
    - [ ] 对进程间通信核心服务进行单元测试
    - [ ] 集成进程管理、内存管理和进程间通信核心服务
    - [ ] 对集成后的核心服务进行系统测试
  - [ ] 基于消息传递的进程间通信（IPC）机制
  - [ ] 动态模块加载器（ELF格式内核模块支持）
  - [ ] 运行时模块热插拔配置系统
  - [ ] 微内核验证测试框架（基于QEMU）
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
  - [ ] 支持Windows可执行文件运行
    - [ ] 研究.exe、.dll、.com、.bat、.cmd、.msi、.scr、.appx、.msix文件的运行机制和依赖环境
    - [ ] 实现这些文件在Lilith OS上的模拟运行环境
    - [ ] 进行这些文件的兼容性测试
  - [ ] 支持macOS应用文件运行
    - [ ] 了解.app、.dmg、.pkg文件的特性和运行要求
    - [ ] 构建这些文件在Lilith OS上的适配环境
    - [ ] 完成这些文件的稳定性测试
  - [ ] 支持iOS应用文件运行
    - [ ] 分析.ipa文件的结构和运行原理
    - [ ] 搭建.ipa文件在Lilith OS上的运行环境
    - [ ] 开展.ipa文件的功能测试
  - [ ] 支持Android应用文件运行
    - [ ] 分析.apk文件的结构和运行原理
    - [ ] 搭建.apk文件在Lilith OS上的运行环境
    - [ ] 开展.apk文件的功能测试
  - [ ] 支持Linux脚本和库文件使用
    - [ ] 学习.sh、.so文件的执行和加载机制
    - [ ] 实现这些文件在Lilith OS上的支持功能
    - [ ] 进行这些文件的功能测试
  - [ ] 支持Linux软件包安装
    - [ ] 学习.deb、.rpm文件的安装流程和依赖管理
    - [ ] 实现这些文件在Lilith OS上的安装功能
    - [ ] 进行这些文件的安装测试
  - [ ] 支持压缩文件处理
    - [ ] 掌握.tar.gz、.tar.bz2、7z、.zip、.rar文件的解压机制
    - [ ] 开发这些文件在Lilith OS上的解压工具
    - [ ] 开展这些文件的解压验证
- 🚧 物理内存管理
- 🚧 QEMU集成测试
- 🚧 异步驱动框架
- 🚧 硬件加速支持（GPU/Vulkan）

## 测试

```bash
cargo test
```

## 快速开始

```bash
# 编译
cargo build

# 运行
cargo run

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

/
├── lilith-kernel/     # 系统内核
│   ├── src/    # 内核源代码
│   ├── Makefile
│   └── ...

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
