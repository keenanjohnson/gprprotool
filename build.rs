use std::path::PathBuf;
use std::process::Command;

fn main() {
    let gpr_dir = PathBuf::from("vendor/gpr");

    if !gpr_dir.exists() {
        panic!("GPR library not found at vendor/gpr. Please run: git clone https://github.com/gopro/gpr.git vendor/gpr");
    }

    // Build GPR library using CMake
    let build_dir = gpr_dir.join("build");

    // Create build directory if it doesn't exist
    if !build_dir.exists() {
        std::fs::create_dir(&build_dir).expect("Failed to create build directory");
    }

    // Run CMake to configure
    let cmake_status = Command::new("cmake")
        .current_dir(&build_dir)
        .arg("..")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .status();

    match cmake_status {
        Ok(status) if status.success() => {
            println!("cargo:warning=CMake configuration successful");
        }
        Ok(status) => {
            panic!("CMake configuration failed with status: {}", status);
        }
        Err(e) => {
            panic!("Failed to run CMake. Is CMake installed? Error: {}", e);
        }
    }

    // Build with CMake
    let build_status = Command::new("cmake")
        .current_dir(&build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg("Release")
        .status();

    match build_status {
        Ok(status) if status.success() => {
            println!("cargo:warning=GPR library built successfully");
        }
        Ok(status) => {
            panic!("GPR library build failed with status: {}", status);
        }
        Err(e) => {
            panic!("Failed to build GPR library: {}", e);
        }
    }

    // Tell Cargo where to find the libraries
    let lib_dir = build_dir.join("source/lib");
    println!("cargo:rustc-link-search=native={}/gpr_sdk", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/common", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/vc5_decoder", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/vc5_common", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/dng_sdk", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/xmp_core", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/expat_lib", lib_dir.display());
    println!("cargo:rustc-link-search=native={}/md5_lib", lib_dir.display());

    // Link the libraries
    println!("cargo:rustc-link-lib=static=gpr_sdk");
    println!("cargo:rustc-link-lib=static=common");
    println!("cargo:rustc-link-lib=static=vc5_decoder");
    println!("cargo:rustc-link-lib=static=vc5_common");
    println!("cargo:rustc-link-lib=static=dng_sdk");
    println!("cargo:rustc-link-lib=static=xmp_core");
    println!("cargo:rustc-link-lib=static=expat_lib");
    println!("cargo:rustc-link-lib=static=md5_lib");

    // Link C++ standard library
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=dylib=msvcrt");

    println!("cargo:rerun-if-changed=vendor/gpr");
}
