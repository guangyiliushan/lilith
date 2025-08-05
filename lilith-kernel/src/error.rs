//! 错误类型定义
//! 
//! 本模块定义了内核中使用的各种错误类型

use core::fmt;

/// 内核通用错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelError {
    /// 内存不足
    OutOfMemory,
    /// 无效参数
    InvalidArgument,
    /// 权限不足
    PermissionDenied,
    /// 资源忙
    ResourceBusy,
    /// 资源不存在
    NotFound,
    /// 操作不支持
    NotSupported,
    /// 设备错误
    DeviceError,
    /// 网络错误
    NetworkError,
    /// 文件系统错误
    FilesystemError,
}

/// 引导过程错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    /// 硬件不兼容
    HardwareIncompatible,
    /// 配置错误
    ConfigurationError,
    /// 内存检测失败
    MemoryDetectionFailed,
    /// 设备初始化失败
    DeviceInitializationFailed,
    /// 寄存器配置失败
    RegisterConfigurationFailed,
}

/// 内存管理错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    /// 内存不足
    OutOfMemory,
    /// 地址无效
    InvalidAddress,
    /// 权限不足
    PermissionDenied,
    /// 页面错误
    PageFault,
    /// 对齐错误
    AlignmentError,
}

/// 调度器错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerError {
    /// 进程不存在
    ProcessNotFound,
    /// 无效的进程状态
    InvalidProcessState,
    /// 调度队列满
    ScheduleQueueFull,
    /// 优先级无效
    InvalidPriority,
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::OutOfMemory => write!(f, "内存不足"),
            KernelError::InvalidArgument => write!(f, "无效参数"),
            KernelError::PermissionDenied => write!(f, "权限不足"),
            KernelError::ResourceBusy => write!(f, "资源忙"),
            KernelError::NotFound => write!(f, "资源不存在"),
            KernelError::NotSupported => write!(f, "操作不支持"),
            KernelError::DeviceError => write!(f, "设备错误"),
            KernelError::NetworkError => write!(f, "网络错误"),
            KernelError::FilesystemError => write!(f, "文件系统错误"),
        }
    }
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootError::HardwareIncompatible => write!(f, "硬件不兼容"),
            BootError::ConfigurationError => write!(f, "配置错误"),
            BootError::MemoryDetectionFailed => write!(f, "内存检测失败"),
            BootError::DeviceInitializationFailed => write!(f, "设备初始化失败"),
            BootError::RegisterConfigurationFailed => write!(f, "寄存器配置失败"),
        }
    }
}

impl From<BootError> for KernelError {
    fn from(err: BootError) -> Self {
        match err {
            BootError::HardwareIncompatible => KernelError::NotSupported,
            BootError::ConfigurationError => KernelError::InvalidArgument,
            BootError::MemoryDetectionFailed => KernelError::OutOfMemory,
            BootError::DeviceInitializationFailed => KernelError::DeviceError,
            BootError::RegisterConfigurationFailed => KernelError::DeviceError,
        }
    }
}

impl From<MemoryError> for KernelError {
    fn from(err: MemoryError) -> Self {
        match err {
            MemoryError::OutOfMemory => KernelError::OutOfMemory,
            MemoryError::InvalidAddress => KernelError::InvalidArgument,
            MemoryError::PermissionDenied => KernelError::PermissionDenied,
            MemoryError::PageFault => KernelError::DeviceError,
            MemoryError::AlignmentError => KernelError::InvalidArgument,
        }
    }
}