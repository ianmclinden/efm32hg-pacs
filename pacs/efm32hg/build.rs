use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32HG108").is_some() {
            "src/efm32hg108/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG110").is_some() {
            "src/efm32hg110/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG210").is_some() {
            "src/efm32hg210/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG222").is_some() {
            "src/efm32hg222/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG308").is_some() {
            "src/efm32hg308/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG309").is_some() {
            "src/efm32hg309/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG310").is_some() {
            "src/efm32hg310/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG321").is_some() {
            "src/efm32hg321/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG322").is_some() {
            "src/efm32hg322/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32HG350").is_some() {
            "src/efm32hg350/device.x"
        } else {
            panic!("No device features selected");
        };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
