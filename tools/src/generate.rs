//! Generate PAC sources from the SVD files.
//!
//! Run with
//! `cargo run --manifest-path tools/Cargo.toml --release --bin generate`
//!
//! Note: Only the F32 variant is used, the F64 is skipped,

use std::{fs, io::Read, path::PathBuf, process::Command};

use form::create_directory_structure;
use svd2rust::{generate, Generation};
use svdtools::patch::{self, EncoderConfig};

/// The device family prefix, used for finding SVDs and in output naming
const FAMILY_PREFIX: &str = "EFM32HG";
/// The device variant suffix, used for finding SVDs and in output naming
const VAR_SUFFIX: &str = "F32";
/// Directory containing SVDs, relative to the repo root
const SVD_DIR: &str = "svd/EFM32HG";

/// Path to the base DEVINFO patch, relative to the repo root
const BASE_PATCH: &str = "patch/EFM32HG/EFM32HG.yaml";
/// Path to the DAC patch, relative to the repo root
const DAC_PATCH: &str = "patch/EFM32HG/EFM32HG-DAC.yaml";
/// Most variants use both patches, but efm32hg108 & efm32hg308 have no ADC or IDAC,
/// so they don't use the additional patch
const DAC_PATCH_VARS: &[&str] = &["110", "210", "222", "309", "310", "321", "322", "350"];

fn main() {
    let base_patch = PathBuf::from(BASE_PATCH);
    assert!(
        base_patch.exists(),
        "Cannot find patch: {}",
        base_patch.display()
    );
    let dac_patch = PathBuf::from(DAC_PATCH);
    assert!(
        dac_patch.exists(),
        "Cannot find patch: {}",
        dac_patch.display()
    );
    let svd_prefix = FAMILY_PREFIX.to_ascii_lowercase();

    let mut svds: Vec<_> = fs::read_dir(SVD_DIR)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            let file_str = path.file_name().unwrap().to_str().unwrap();
            file_str.starts_with(FAMILY_PREFIX) && file_str.ends_with(&format!("{VAR_SUFFIX}.svd"))
        })
        .collect();
    svds.sort();

    // contents of the first generic.rs generated
    let mut generic: Option<String> = None;

    for svd in svds {
        let name = svd.file_stem().unwrap().to_str().unwrap(); // e.g. EFM32HG309F32
        let Some(dev) = name
            .strip_prefix(FAMILY_PREFIX)
            .unwrap()
            .strip_suffix(VAR_SUFFIX)
        else {
            continue;
        };
        println!("Generating {svd_prefix}{dev} from {:?}", svd.display());

        let mut xml = fs::read_to_string(&svd).unwrap();

        println!("  - patching with: {}", base_patch.display());
        let patch = patch::load_patch(&base_patch).unwrap();
        let (encoder, config) = (EncoderConfig::default(), patch::Config::default());
        let mut patched = String::new();
        patch::process_reader(xml.as_bytes(), &patch, &encoder, &config)
            .unwrap()
            .read_to_string(&mut patched)
            .unwrap();
        xml = patched;

        if DAC_PATCH_VARS.contains(&dev) {
            println!("  - patching with: {}", dac_patch.display());
            let patch = patch::load_patch(&dac_patch).unwrap();
            let mut patched = String::new();
            patch::process_reader(xml.as_bytes(), &patch, &encoder, &config)
                .unwrap()
                .read_to_string(&mut patched)
                .unwrap();
            xml = patched;
        }

        // svd2rust writes generic.rs into output_dir
        let tmp = format!("target/generated/{dev}");
        fs::create_dir_all(format!("{tmp}/src")).unwrap();
        let config = svd2rust::Config {
            atomics: true,
            make_mod: true,
            generic_mod: true,
            output_dir: tmp.clone().into(),
            ..svd2rust::Config::default()
        };
        let Generation {
            lib_rs,
            device_specific,
            ..
        } = generate(&xml, &config).unwrap();

        let new_generic = fs::read_to_string(format!("{tmp}/generic.rs")).unwrap();
        if let Some(existing) = generic.as_ref() {
            assert!(
                existing == &new_generic,
                "generic.rs changed between hardware variants"
            );
        } else {
            generic.replace(new_generic);
        }

        // generate library, add device.x
        create_directory_structure(format!("{tmp}/src"), &lib_rs).unwrap();
        fs::rename(format!("{tmp}/src/lib.rs"), format!("{tmp}/src/mod.rs")).unwrap();
        let device_x = device_specific.unwrap().device_x;
        fs::write(format!("{tmp}/src/device.x"), format!("{device_x}\n")).unwrap();

        // copy to output directory
        let dest = format!("pacs/{svd_prefix}/src/{svd_prefix}{dev}");
        let _ = fs::remove_dir_all(&dest).ok();
        fs::rename(format!("{tmp}/src"), dest).unwrap();
    }

    let generic = generic.take().expect("No SVD generated a valid generic.rs");
    fs::write(format!("pacs/{svd_prefix}/src/generic.rs"), generic).unwrap();
    let _ = fs::remove_dir_all("target/generated").ok();

    println!("Running cargo format");
    Command::new("cargo")
        .args([
            "fmt",
            "--manifest-path",
            &format!("pacs/{svd_prefix}/Cargo.toml"),
        ])
        .status()
        .unwrap();
}
