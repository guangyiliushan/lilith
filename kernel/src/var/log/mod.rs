use alloc::sync::Arc;
use crate::vfs::{FileSystem, Inode, FileType, Metadata};

pub struct LogFS;

impl FileSystem for LogFS {
    fn root_inode(&self) -> Arc<dyn Inode> {
        Arc::new(LogRootDir::new())
    }
}

struct LogRootDir {
    entries: spin::Mutex<alloc::vec::Vec<alloc::string::String>>,
}

impl LogRootDir {
    fn new() -> Self {
        LogRootDir {
            entries: spin::Mutex::new(alloc::vec!["kernel.log".into()]),
        }
    }
}

impl Inode for LogRootDir {
    fn read_dir(&self, _offset: usize) -> crate::vfs::Result<alloc::vec::Vec<(alloc::string::String, FileType)>> {
        let guard = self.entries.lock();
        Ok(guard.iter().map(|name| (name.clone(), FileType::File)).collect())
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