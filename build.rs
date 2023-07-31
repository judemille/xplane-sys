use std::path::PathBuf;
use std::{env, str::FromStr};
use target_lexicon::{Architecture, OperatingSystem, Triple};

fn get_clang_args(crate_path: &PathBuf) -> Vec<String> {
    let mut r = Vec::new();
    r.push("-DLIN".to_string()); // Technically tells the headers they're being compiled for Linux.
                                 // Doesn't matter for our use case -- the only things that are changed are irrelevant to bindgen.
    if cfg!(not(any(feature = "XPLM200", feature = "OLD"))) {
        panic!("Please set a desired SDK version!");
    }
    if cfg!(all(feature = "XPLM200", feature = "OLD")) {
        panic!("Using both normal version features and the OLD feature is paradoxical. Pick one.");
    }
    if cfg!(feature = "XPLM400") {
        r.push("-DXPLM400".to_string());
    }
    if cfg!(feature = "XPLM303") {
        r.push("-DXPLM303".to_string());
    }
    if cfg!(feature = "XPLM301") {
        r.push("-DXPLM301".to_string());
    }
    if cfg!(feature = "XPLM300") {
        r.push("-DXPLM300".to_string());
    }
    if cfg!(feature = "XPLM210") {
        r.push("-DXPLM210".to_string());
    }
    if cfg!(feature = "XPLM200") {
        r.push("-DXPLM200".to_string());
    }

    let cheaders = crate_path.join("XPlaneSDK/CHeaders");
    let xplmheaders = cheaders.join("XPLM");
    let widgetheaders = cheaders.join("Widgets");
    r.push(format!("-I{}", xplmheaders.to_str().unwrap()));
    r.push(format!("-I{}", widgetheaders.to_str().unwrap()));

    r
}

fn handle_platform(crate_path: PathBuf) {
    let target = env::var("TARGET").unwrap();
    let triple = Triple::from_str(&target).unwrap();
    match triple.operating_system {
        OperatingSystem::Windows => {
            match triple.architecture {
                Architecture::X86_64 => {},
                _ => panic!("Unsupported target architecture! xplane-sys can only be compiled for x86_64 on Windows!")
            };
            let library_path = crate_path.join("SDK/Libraries/Win");
            println!("cargo:rustc-link-search={}", library_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=XPLM_64");
            println!("cargo:rustc-link-lib=XPWidgets_64");
        },
        OperatingSystem::MacOSX {..} => {
            match triple.architecture {
                Architecture::Aarch64(_) | Architecture::X86_64 => {},
                _ => panic!("Unsupported target architecture! xplane-sys can only be compiled for x86_64 or aarch64 on Mac!")
            };
            let library_path = crate_path.join("SDK/Libraries/Mac");
            println!("cargo:rustc-link-search-framework=framework={}", library_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=framework=XPLM");
            println!("cargo:rustc-link-lib=framework=XPWidgets");
        },
        OperatingSystem::Linux => {
            match triple.architecture {
                Architecture::X86_64 => {},
                _ => panic!("Unsupported target architecture! xplane-sys can only be compiled for x86_64 on Linux!")
            }
        }, // No need to link libraries on Linux.
        _ => panic!("Unsupported operating system! xplane-sys can only be compiled for Windows, Mac, or Linux!")
    };
}

fn main() {
    // Get the absolute path to this crate, so that linking will work when done in another folder
    let crate_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("src/combined.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(get_clang_args(&crate_path))
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    handle_platform(crate_path);
    /*     if target.contains("-apple-") {
        let library_path = crate_path.join("SDK/Libraries/Mac");
        println!(
            "cargo:rustc-link-search=framework={}",
            library_path.to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=framework=XPLM");
        println!("cargo:rustc-link-lib=framework=XPWidgets");
    } else if target.contains("-linux-") {
        // Do nothing for Linux
    } else if target.contains("-windows-") {
        let library_path = crate_path.join("SDK/Libraries/Win");
        println!("cargo:rustc-link-search={}", library_path.to_str().unwrap());
        if target.contains("x86_64") {
            println!("cargo:rustc-link-lib=XPLM_64");
            println!("cargo:rustc-link-lib=XPWidgets_64");
        } else {
            println!("cargo:rustc-link-lib=XPLM");
            println!("cargo:rustc-link-lib=XPWidgets");
        }
    } else {
        panic!("Target operating system not Mac OS, Linux, or Windows. As of the writing of this version of this crate, X-Plane does not support any other platform.")
    } */
}
