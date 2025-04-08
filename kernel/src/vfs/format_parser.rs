use alloc::string::String;

#[derive(Debug, Clone, Copy)]
pub enum ExecFormat {
    PE,
    ELF,
    MachO,
    COM,
    Script,
    Unknown
}

pub fn identify_exec_format(header: &[u8]) -> ExecFormat {
    match header {
        // PE格式校验(MZ头 + PE\0\0)
        [0x4D, 0x5A, ..] if header.len() > 0x3C && 
            header.get(header[0x3C] as usize..).map_or(false, |pe| 
                pe.starts_with(&[0x50, 0x45, 0x00, 0x00])) => ExecFormat::PE,
        // ELF格式校验(包含e_type检查)
        [0x7F, 0x45, 0x4C, 0x46, ref rest @ ..] => match rest.get(16) {
            Some(0x02) | Some(0x03) => ExecFormat::ELF,  // ET_EXEC(0x02) 或 ET_DYN(0x03)
            _ => ExecFormat::Unknown
        },
        // Mach-O多架构支持
        [0xFE, 0xED, 0xFA, 0xCE, ..] | [0xFE, 0xED, 0xFA, 0xCF, ..] |  // 32位
        [0xCF, 0xFA, 0xED, 0xFE, ..] | [0xCE, 0xFA, 0xED, 0xFE, ..] => ExecFormat::MachO,  // 64位小端
        // DOS COM文件
        [0xEB, ..] if header.len() > 1 && header[1] < 0x10 => ExecFormat::COM,
        // 脚本文件
        _ if String::from_utf8_lossy(header).contains("#!/") => ExecFormat::Script,
        _ => ExecFormat::Unknown
    }
}

pub fn get_format_name(format: ExecFormat) -> &'static str {
    match format {
        ExecFormat::PE => "PE",
        ExecFormat::ELF => "ELF",
        ExecFormat::MachO => "Mach-O",
        ExecFormat::COM => "COM",
        ExecFormat::Script => "Script",
        ExecFormat::Unknown => "Unknown"
    }
}