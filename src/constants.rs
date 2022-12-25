#[cfg(windows)]
pub const EXEC_EXT: &str = ".cmd";
#[cfg(not(windows))]
pub const EXEC_EXT: &str = "";

#[cfg(target_os = "windows")]
pub const PLATFORM: &str = "win";
#[cfg(target_os = "macos")]
pub const PLATFORM: &str = "darwin";
#[cfg(target_os = "linux")]
pub const PLATFORM: &str = "linux";

#[cfg(target_os = "windows")]
pub const EXT: &str = ".zip";
#[cfg(target_os = "macos")]
pub const EXT: &str = ".tar.gz";
#[cfg(target_os = "linux")]
pub const EXT: &str = ".tar.gz";

#[cfg(target_arch = "x86_64")]
pub const ARCH: &str = "x64";
#[cfg(target_arch = "x86")]
pub const ARCH: &str = "x86";
#[cfg(target_arch = "aarch64")]
pub const ARCH: &str = "arm64";

pub const X64: &str = "x64";
