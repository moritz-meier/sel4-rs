use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use indoc::formatdoc;

fn main() -> Result<()> {
    let out_dir: PathBuf = env::var("OUT_DIR")?.into();
    // let sel4_dir: PathBuf = env::var("DEP_SEL4_KERNEL_DIR")?.into();
    // let build_dir: PathBuf = env::var("DEP_SEL4_KERNEL_BUILD_DIR")?.into();

    copy_linker_script("kernel-armv7a.ld", &out_dir)?;
    create_memory_ld(&out_dir)?;

    println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-arg=-Tkernel.ld");

    Ok(())
}

fn copy_linker_script(linker_script: &str, out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();

    let ldscripts: PathBuf = "ldscripts".into();
    let kernel_ld = ldscripts.join(linker_script);

    fs::copy(kernel_ld, out_dir.join("kernel.ld"))?;

    Ok(())
}

fn create_memory_ld(out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();

    let memory = formatdoc! {
        r#"
        "#
    };

    fs::write(out_dir.join("memory.ld"), memory)?;

    Ok(())
}
