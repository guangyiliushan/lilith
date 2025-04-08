#![no_std]
#![no_main]

use vfs::{FileType, VFS};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut fs = VFS::new();
    
    // 创建Linux标准目录结构
    let root = &mut fs.root;
    fs.create_dir(root, "bin");
    fs.create_dir(root, "etc");
    fs.create_dir(root, "dev");
    fs.create_dir(root, "proc");
    fs.create_dir(root, "sys");
    fs.create_dir(root, "home");
    fs.create_dir(root, "tmp");
    fs.create_dir(root, "usr");
    fs.create_dir(root, "var");
    fs.create_dir(root, "boot");
    fs.create_dir(root, "lib");
    fs.create_dir(root, "mnt");
    fs.create_dir(root, "opt");
    fs.create_dir(root, "run");

    // 初始化标准设备
    let dev_dir = root.children.get_mut("dev").unwrap();
    dev_dir.add_child(Inode::new_device(12, "null", 1, 3, &NullDevice));
    dev_dir.add_child(Inode::new_device(13, "zero", 1, 5, &ZeroDevice));
    dev_dir.add_child(Inode::new_device(14, "random", 1, 8, &RandomDevice));
    
    loop {}
}

struct NullDevice;
struct ZeroDevice;
struct RandomDevice;

impl FileOperations for NullDevice {
    fn write(&self, _offset: u64, _buf: &[u8]) -> usize { 0 }
}

impl FileOperations for ZeroDevice {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> usize {
        buf.fill(0);
        buf.len()
    }
}

impl FileOperations for RandomDevice {
    fn read(&self, _offset: u64, buf: &mut [u8]) -> usize {
        // 简单随机数生成
        for byte in buf.iter_mut() {
            *byte = rand::random();
        }
        buf.len()
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}