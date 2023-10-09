use bindgen::callbacks::ParseCallbacks;
use bindgen::EnumVariation;
use std::path::{Path, PathBuf};
use std::{env, str::FromStr};
use target_lexicon::{Architecture, Environment, OperatingSystem, Triple};

fn get_clang_args(crate_path: &Path) -> Vec<String> {
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
            if triple.architecture != Architecture::X86_64 {
                panic!(
                    "Unsupported target architecture! xplane-sys on Windows only supports x86_64."
                );
            }
            if triple.environment != Environment::Msvc {
                panic!(
                    "Unsupported environment! X-Plane uses the MSVC ABI. Compile for that target."
                );
            }
            let library_path = crate_path.join("SDK/Libraries/Win");
            println!("cargo:rustc-link-search={}", library_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=XPLM_64");
            println!("cargo:rustc-link-lib=XPWidgets_64");
        }
        OperatingSystem::MacOSX { .. } => {
            match triple.architecture {
                Architecture::Aarch64(_) | Architecture::X86_64 => {},
                _ => panic!("Unsupported target architecture! xplane-sys on Mac only supports x86_64 or aarch64.")
            };
            let library_path = crate_path.join("SDK/Libraries/Mac");
            println!(
                "cargo:rustc-link-search-framework=framework={}",
                library_path.to_str().unwrap()
            );
            println!("cargo:rustc-link-lib=framework=XPLM");
            println!("cargo:rustc-link-lib=framework=XPWidgets");
        }
        OperatingSystem::Linux => {
            if triple.architecture != Architecture::X86_64 {
                panic!(
                    "Unsupported target architecture! xplane-sys on Linux only supports x86_64."
                );
            }
            if triple.environment != Environment::Gnu {
                panic!("Unsupported environment! X-Plane runs on the GNU ABI on Linux, and so xplane-sys requires a GNU target.");
            }
        } // No need to link libraries on Linux.
        _ => panic!(
            "Unsupported operating system! The X-Plane SDK only supports Windows, Mac, and Linux."
        ),
    };
}

#[derive(Debug)]
struct EnumHandler;
impl ParseCallbacks for EnumHandler {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let mut out = original_variant_name
            .trim_start_matches("xplm")
            .trim_start_matches('_');
        let Some(enum_name) = enum_name else {
            return Some(out.to_string());
        };
        out = match enum_name {
            "XPLMCameraControlDuration" => out.trim_start_matches("ControlCamera"),
            "XPLMDataTypeID" => out.trim_start_matches("Type"),
            "XPLMKeyFlags" => out.trim_end_matches("Flag"),
            "XPLMDrawingPhase" => out.trim_start_matches("Phase"),
            "XPLMDeviceID" => {
                out = if out.ends_with("_1") || out.ends_with("_2") || out.ends_with("_3") {
                    let side = out.chars().last().unwrap(); // We know this string ends with something.
                    let side = match side {
                        '1' => "Pilot",
                        '2' => "Copilot",
                        '3' => "Center",
                        _ => unreachable!(),
                    };
                    return Some(out[0..out.len() - 1].to_string() + side);
                } else {
                    out
                };
                out.trim_start_matches("device")
            }
            "XPLMMouseStatus" => out.trim_start_matches("Mouse"),
            "XPLMCursorStatus" => out.trim_start_matches("Cursor"),
            "XPLMWindowLayer" => out.trim_start_matches("WindowLayer"),
            "XPLMWindowDecoration" => out.trim_start_matches("WindowDecoration"),
            "XPLMWindowPositioningMode" => out.trim_start_matches("Window"),
            "XPLMFontID" => out.trim_start_matches("Font"),
            "XPLMMapStyle" => out.trim_start_matches("MapStyle"),
            "XPLMMapLayerType" => out.trim_start_matches("MapLayer"),
            "XPLMMapOrientation" => out.trim_start_matches("MapOrientation"),
            "XPLMMenuCheck" => out.trim_start_matches("Menu"),
            "XPLMNavType" => out.trim_start_matches("Nav"),
            "XPLMFlightLoopPhaseType" => out.trim_start_matches("FlightLoop_Phase"),
            "XPLMProbeType" | "XPLMProbeResult" => out.trim_start_matches("Probe"),
            "XPLMAudioBus" => out.trim_start_matches("Audio"),
            "XPLMBankID" => out.trim_end_matches("Bank"),
            "XPLMCommandPhase" => out.trim_start_matches("Command"),
            "XPLMDataFileType" => out.trim_start_matches("DataFile"),
            "XPLMHostApplicationID" => out.trim_start_matches("Host"),
            "XPLMLanguageCode" => out.trim_start_matches("Language"),
            "XPWindowStyle" => out.trim_start_matches("xpWindow"),
            "XPElementStyle" => out.trim_start_matches("xpElement"),
            "XPTrackStyle" => out.trim_start_matches("xpTrack"),
            "XPWidgetPropertyID" => out.trim_start_matches("xpProperty"),
            "XPDispatchMode" => out.trim_start_matches("xpMode"),
            "XPWidgetMessage" => out.trim_start_matches("xpMsg"),
            enum_name => {
                if !enum_name.starts_with("XPLM") && enum_name.starts_with("XP") {
                    out = out.trim_start_matches("xp").trim_start_matches('_');
                    if enum_name.ends_with("Property") {
                        out = out.trim_start_matches("Property");
                    } else if enum_name.ends_with("Message") {
                        out = out.trim_start_matches("Message").trim_start_matches("Msg");
                    } else if enum_name == "XPScrollBarType" {
                        out = out.trim_start_matches("ScrollBarType");
                    } else if enum_name == "XPTextFieldType" {
                        out = out.trim_start_matches("Text");
                    } else if enum_name == "XPButtonBehavior" {
                        out = out.trim_start_matches("ButtonBehavior");
                    } 
                    out.trim_start_matches('_')
                } else {
                    out
                }
            }
        };
        out = out.trim_start_matches('_');
        Some(out.to_string())
    }
}

fn main() {
    // Get the absolute path to this crate, so that linking will work when done in another folder
    let crate_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("src/combined.h")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .bitfield_enum("XPLMDataTypeID")
        .use_core()
        .parse_callbacks(Box::new(EnumHandler))
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
