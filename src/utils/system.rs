#[derive(Debug, PartialEq)]
pub enum OS {
    Windows,
    Linux,
    Unknown,
}

pub fn get_os() -> OS {
    if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "linux") {
        OS::Linux
    } else {
        OS::Unknown
    }
}
