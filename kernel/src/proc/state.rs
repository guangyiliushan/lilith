// 进程状态机实现
use core::fmt;
use super::ProcessId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    /// 新建未初始化
    Created,
    /// 就绪可调度
    Ready,
    /// 正在执行
    Running,
    /// 阻塞等待
    Blocked,
    /// 僵尸状态
    Zombie,
}

#[derive(Debug)]
pub struct ProcessControlBlock {
    pub pid: ProcessId,
    pub state: ProcessState,
    pub context: ProcessContext,
    pub priority: u8,
}

impl ProcessState {
    /// 状态合法性校验
    pub fn transition(&self, new_state: ProcessState) -> Result<(), &'static str> {
        match (self, new_state) {
            (ProcessState::Created, ProcessState::Ready) => Ok(()),
            (ProcessState::Ready, ProcessState::Running) => Ok(()),
            (ProcessState::Running, ProcessState::Ready) => Ok(()),
            (ProcessState::Running, ProcessState::Blocked) => Ok(()),
            (ProcessState::Blocked, ProcessState::Ready) => Ok(()),
            (ProcessState::Running, ProcessState::Zombie) => Ok(()),
            _ => Err("非法状态转换"),
        }
    }

    /// 创建子进程的状态初始化
    pub fn fork_transition(&self) -> Result<ProcessState, &'static str> {
        if *self == ProcessState::Running {
            Ok(ProcessState::Ready)
        } else {
            Err("非运行状态无法fork")
        }
    }

    /// 处理exit系统调用
    pub fn exit_transition(&self) -> ProcessState {
        ProcessState::Zombie
    }
}

/// 进程上下文保存结构
#[derive(Clone)]
pub struct ProcessContext {
    pub instruction_ptr: u64,
    pub stack_ptr: u64,
    pub flags: u64,
    // 其他寄存器...
}

// 原子化状态修改
impl ProcessControlBlock {
    pub fn set_state(&mut self, new_state: ProcessState) -> Result<(), &'static str> {
        self.state.transition(new_state)?;
        self.state = new_state;
        Ok(())
    }
}

lazy_static! {
    /// 全局进程表
    pub static ref PROCESS_TABLE: spin::Mutex<Vec<ProcessControlBlock>> = 
        spin::Mutex::new(Vec::with_capacity(128));
}