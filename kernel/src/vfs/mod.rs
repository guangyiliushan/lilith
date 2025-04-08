use alloc::collections::BTreeMap;
use alloc::string::String;

#[derive(Debug)]
pub enum FileType {
    Directory,
    CharDevice,
    BlockDevice,
    FIFO,
    Socket,
    SymbolicLink,
    RegularFile,
}

pub struct VFS {
    root: Inode,
    next_inode: u64,
}

pub struct Inode {
    pub num: u64,
    pub name: String,
    pub file_type: FileType,
    pub children: BTreeMap<String, Inode>,
    pub device: Option<(u32, u32)>,
    pub ops: Option<&'static dyn FileOperations>,
}

impl VFS {
    pub fn new() -> Self {
        let mut vfs = Self {
            root: Inode::new_dir(0, "/"),
            next_inode: 1,
        };

        // 创建标准Linux目录结构
        let root = &mut vfs.root;
        vfs.create_dir(root, "bin");
        vfs.create_dir(root, "etc");
        vfs.create_dir(root, "dev");
        vfs.create_dir(root, "home");
        vfs.create_dir(root, "proc");
        vfs.create_dir(root, "sys");

        // 初始化proc文件系统
        let proc = vfs.root.children.get_mut("proc").unwrap();
        vfs.create_proc_file(proc, "cpuinfo", || {
            String::from("processor	: 0\nvendor_id	: LilithCPU\ncpu family	: 6\nmodel name	: VFS Processor\n")
        });
        vfs.create_proc_file(proc, "meminfo", || {
            String::from("MemTotal:        2048000 kB\nMemFree:         1024000 kB\n")
        });

        // 初始化sys文件系统
        let sys = vfs.root.children.get_mut("sys").unwrap();
        vfs.create_dir(sys, "devices");
        vfs.create_dir(sys, "module");
        vfs.create_proc_file(sys, "kernel_version", || {
            String::from("Lilith Kernel 1.0.0\n")
        });
        vfs.create_dir(root, "tmp");
        vfs.create_dir(root, "usr");
        vfs.create_dir(root, "var");
        vfs.create_dir(root, "boot");
        vfs.create_dir(root, "lib");
        vfs.create_dir(root, "mnt");
        vfs.create_dir(root, "opt");
        vfs.create_dir(root, "run");
        vfs.create_dir(root, "sbin");

        // 初始化var日志系统
        vfs.init_var_log();

        // 添加基本设备节点
        let dev = vfs.root.children.get_mut("dev").unwrap();
        vfs.create_device(dev, "null", 1, 3);
        vfs.create_device(dev, "zero", 1, 5);
        vfs.create_device(dev, "tty", 5, 0);
        vfs.create_device(dev, "random", 1, 8);
        vfs.create_device(dev, "urandom", 1, 9);

        vfs
    }

    pub fn create_device(&mut self, parent: &mut Inode, name: &str, major: u32, minor: u32) -> &mut Inode {
        let inode = Inode::new_device(self.next_inode, name, major, minor);
        self.next_inode += 1;
        parent.add_child(inode)
    }

    pub fn create_proc_file(&mut self, parent: &mut Inode, name: &str, generator: fn() -> String) -> &mut Inode {
        let inode = Inode::new_proc_file(self.next_inode, name, generator);
        self.next_inode += 1;
        parent.add_child(inode)
    }

    pub fn create_sys_file(&mut self, parent: &mut Inode, name: &str, generator: fn() -> String) -> &mut Inode {
        self.create_proc_file(parent, name, generator)
    }

    pub fn create_dir(&mut self, parent: &mut Inode, name: &str) -> &mut Inode {
        let inode = Inode::new_dir(self.next_inode, name);
        self.next_inode += 1;
        parent.add_child(inode)
    }
}

pub trait FileOperations {
    fn read(&self, offset: u64, buf: &mut [u8]) -> usize;
    fn write(&self, offset: u64, buf: &[u8]) -> usize;
}

struct ProcFile {
    generator: fn() -> String,
}

impl FileOperations for ProcFile {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> usize {
        let content = (self.generator)();
        let bytes = content.as_bytes();
        let len = bytes.len().min(buf.len());
        buf[..len].copy_from_slice(&bytes[..len]);
        len
    }

    fn write(&self, _offset: u64, _buf: &[u8]) -> usize { 0 }
}

impl Inode {
    pub fn new_device(num: u64, name: &str, major: u32, minor: u32) -> Self {
        struct DeviceOps;
        impl FileOperations for DeviceOps {
            fn read(&self, _offset: u64, buf: &mut [u8]) -> usize {
                // 实现设备特定读取逻辑
                buf.fill(0);
                buf.len()
            }
            fn write(&self, _offset: u64, buf: &[u8]) -> usize {
                // 实现设备特定写入逻辑
                buf.len()
            }
        }
        static OPS: DeviceOps = DeviceOps;

        Self {
        Self {
            num,
            name: String::from(name),
            file_type: FileType::CharDevice,
            children: BTreeMap::new(),
            device: Some((major, minor)),
            ops: Some(&OPS),
        }
    }

    fn new_proc_file(num: u64, name: &str, generator: fn() -> String) -> Self {
        static PROCFILE_OPS: ProcFile = ProcFile { generator: || String::new() };
        Self {
            num,
            name: String::from(name),
            file_type: FileType::RegularFile,
            children: BTreeMap::new(),
            device: None,
            ops: Some(&PROCFILE_OPS),
        }
    }

    fn new_dir(num: u64, name: &str) -> Self {
        Self {
            num,
            name: String::from(name),
            file_type: FileType::Directory,
            children: BTreeMap::new(),
            device: None,
            ops: None,
        }
    }

    fn add_child(&mut self, mut child: Inode) -> &mut Inode {
        let name = child.name.clone();
        self.children.insert(name, child);
        self.children.get_mut(&name).unwrap()
    }
}