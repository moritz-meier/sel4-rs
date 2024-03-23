use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use indoc::formatdoc;

fn main() -> Result<()> {
    let out_dir: PathBuf = env::var("OUT_DIR")?.into();
    // let sel4_dir: PathBuf = env::var("DEP_SEL4_KERNEL_DIR")?.into();
    let build_dir: PathBuf = env::var("DEP_SEL4_KERNEL_BUILD_DIR")?.into();

    copy_devicetree(&build_dir, &out_dir)?;
    copy_linker_script("kernel-armv7a.ld", &out_dir)?;
    create_memory_ld(&build_dir, &out_dir)?;

    export_artifacts(build_dir, &out_dir)?;

    println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-arg=-Tkernel.ld");
    println!("cargo:rustc-link-arg=--no-gc-sections");

    Ok(())
}

fn copy_devicetree(build_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();
    let out_dir = out_dir.as_ref();

    let kernel_dtb = build_dir.join("kernel/kernel.dtb");
    fs::copy(kernel_dtb, out_dir.join("kernel.dtb"))?;

    Ok(())
}

fn copy_linker_script(linker_script: &str, out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();

    let ldscripts: PathBuf = "ldscripts".into();
    let kernel_ld = ldscripts.join(linker_script);

    fs::copy(kernel_ld, out_dir.join("kernel.ld"))?;

    Ok(())
}

fn create_memory_ld(build_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();
    let out_dir = out_dir.as_ref();

    let linker_script = fs::read_to_string(build_dir.join("kernel/linker.lds_pp"))?;
    let linker_lines = linker_script.lines();

    let kernel_offset_line = linker_lines
        .into_iter()
        .find(|line| line.trim().starts_with("KERNEL_OFFSET = "))
        .expect("Constant \"KERNEL_OFFSET\" not found in linker script.");

    let (_, kernel_offset_expr) = kernel_offset_line.split_once('=').unwrap();

    let (kernel_virt_base_expr, kernel_phys_base_expr) =
        kernel_offset_expr.trim().rsplit_once('-').unwrap();

    let kernel_virt_base_expr = kernel_virt_base_expr.trim();
    let kernel_phys_base_expr = kernel_phys_base_expr.trim_end_matches(';').trim();

    let memory = formatdoc! {
        r#"
        __KERNEL_PHYS_BASE_ADDR = {};
        __KERNEL_VIRT_BASE_ADDR = {};
        "#,
        kernel_phys_base_expr,
        kernel_virt_base_expr
    };

    fs::write(out_dir.join("memory.ld"), memory)?;

    Ok(())
}

fn export_artifacts(build_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();
    let out_dir = out_dir.as_ref();

    // todo: CARGO_TARGET_DIR
    let cargo_target_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let kernel_dtb = build_dir.join("kernel/kernel.dtb");
    fs::copy(kernel_dtb, cargo_target_dir.join("kernel.dtb"))?;

    Ok(())
}
