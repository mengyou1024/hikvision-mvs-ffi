use std::env;
use std::path::PathBuf;

/// Windows架构
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WindowsArch {
    /// 32位 x86(i686)
    X86,
    // 64位 x86-64 (AMD64)
    X86_64,
    // 其他架构(如 arm64)
    Other,
}

fn main() {
    if std::env::consts::OS != "windows" {
        panic!("This build script is only intended for Windows platforms.");
    }

    let target = env::var("TARGET").expect("TARGET environment variable not set");

    let arch = if target.contains("x86_64") {
        WindowsArch::X86_64
    } else if target.contains("i686") {
        WindowsArch::X86
    } else {
        WindowsArch::Other
    };

    let lib_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(match arch {
        WindowsArch::X86 => "Libraries/Win32",
        WindowsArch::X86_64 => "Libraries/Win64",
        WindowsArch::Other => panic!("Unsupported target architecture :{target}"),
    });
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=MvCameraControl");
}
