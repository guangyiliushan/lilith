use alloc::sync::Arc;
use crate::vfs::{FileSystem, Inode, FileType, Metadata};
use spin::Mutex;
use x86_64::structures::tss::TaskStateSegment;

use super::vfs::format_parser;

pub struct ProcFS;

impl FileSystem for ProcFS {
    fn root_inode(&self) -> Arc<dyn Inode> {
        Arc::new(ProcRootDir::new())
    }
}

struct ProcRootDir;

impl ProcRootDir {
    fn new() -> Self {
        ProcRootDir
    }
}

impl Inode for ProcRootDir {
    fn read_dir(&self, _offset: usize) -> crate::vfs::Result<alloc::vec::Vec<(alloc::string::String, FileType)>> {
        let mut entries = vec![
            ("self".into(), FileType::Dir),
            ("cpuinfo".into(), FileType::File),
        ];
        Ok(entries)
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            file_type: FileType::Dir,
            size: 0,
            block_size: 0,
            blocks: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
}

#[derive(Debug)]
pub struct ProcessContext {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    // ... 其他寄存器状态
}

pub struct Process {
    pub pid: u64,
    pub state: ProcessState,
    pub context: ProcessContext,
    pub kernel_stack: [u8; 4096],
}

pub struct Scheduler {
    ready_queue: alloc::collections::VecDeque<Arc<Mutex<Process>>>,
    current: Option<Arc<Mutex<Process>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: alloc::collections::VecDeque::new(),
            current: None,
        }
    }

    pub fn add_process(&mut self, process: Arc<Mutex<Process>>) {
        self.ready_queue.push_back(process);
    }

    pub fn run(&mut self) {
        if let Some(next) = self.ready_queue.pop_front() {
            let prev = self.current.replace(next.clone());
            if let Some(prev) = prev {
                self.ready_queue.push_back(prev);
            }
            unsafe { switch_context(&mut next.lock().context) };
        }
    }
}

#[naked]
unsafe extern "C" fn switch_context(new_context: &mut ProcessContext) {
    asm!(
        "mov [rdi + 0x00], rsp",
        "mov rsp, [rdi + 0x08]",
        "ret",
        options(noreturn)
    )
}


pub fn load_executable(path: &str) -> Result<ProcessControlBlock, Error> {
    use crate::vfs::filetype_registry;
    
    let file = vfs::open(path)?;
    let metadata = vfs::metadata(&file)?;
    
    // 从文件类型注册表获取匹配的解析器
    let parser = filetype_registry::find_best_parser(&metadata)
        .ok_or_else(|| Error::new(UnsupportedFormat))?;
    
    // 动态派发到对应格式的加载器
    match parser.format_type {
        filetype_registry::ExecFormatType::PE => parse_pe_header(&file),
        filetype_registry::ExecFormatType::ELF => elf::load(file),
        filetype_registry::ExecFormatType::MachO => macho::load(file),
        filetype_registry::ExecFormatType::Script => launch_interpreter(&file),
        _ => Err(Error::new(UnsupportedFormat))
    }
}