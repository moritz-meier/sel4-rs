use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use duct::*;

fn main() -> Result<()> {
    let out_dir: PathBuf = env::var("OUT_DIR")?.into();
    let sel4_dir: PathBuf = env::var("DEP_SEL4_DIR")?.into();
    let build_dir: PathBuf = env::var("DEP_SEL4_BUILD_DIR")?.into();

    println!(
        "cargo:DIR={}",
        sel4_dir.to_str().ok_or(anyhow!("Invalid path"))?
    );

    println!(
        "cargo:BUILD_DIR={}",
        build_dir.to_str().ok_or(anyhow!("Invalid path"))?
    );

    objcopy_devtree(&build_dir, &out_dir)?;
    link_kernel(&build_dir, &out_dir)?;
    redefine_start_sym(&out_dir)?;
    prefix_sections(&out_dir)?;

    println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static:+whole-archive=kernel");

    Ok(())
}

fn objcopy_devtree(build_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();
    let out_dir = out_dir.as_ref();

    let kernel_dtb = build_dir.join("kernel/kernel.dtb");
    let kernel_dtb_obj = out_dir.join("kernel.dtb.obj");

    let out_type = match () {
        #[cfg(feature = "stm32mp1")]
        () => "elf32-littlearm",
        #[cfg(feature = "zynq7000")]
        () => "elf32-littlearm",
    };

    let args = vec![
        format!("-I"),
        format!("binary"),
        format!("-O"),
        format!("{}", out_type),
        format!("--rename-section"),
        format!(".data=.devicetree,alloc,load"),
        format!("{}", kernel_dtb.to_str().unwrap()),
        format!("{}", kernel_dtb_obj.to_str().unwrap()),
    ];

    let output = cmd("arm-unknown-linux-gnueabi-objcopy", args).read()?;

    for line in output.lines() {
        println!("cargo:warning={}", line);
    }

    Ok(())
}

fn link_kernel(build_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();
    let out_dir = out_dir.as_ref();

    let (arch, archv, armv, mode) = match () {
        #[cfg(feature = "stm32mp1")]
        () => ("arm", "armv", "armv7-a", "32"),
        #[cfg(feature = "zynq7000")]
        () => ("arm", "armv", "armv7-a", "32"),
    };

    let mut obj_files = vec![
        build_dir.join("kernel/CMakeFiles/kernel.elf.dir/kernel_all.c.obj"),
        build_dir.join(format!(
            "kernel/CMakeFiles/kernel.elf.dir/src/arch/{arch}/{mode}/head.S.obj"
        )),
        build_dir.join(format!(
            "kernel/CMakeFiles/kernel.elf.dir/src/arch/{arch}/{mode}/hyp_traps.S.obj"
        )),
        build_dir.join(format!(
            "kernel/CMakeFiles/kernel.elf.dir/src/arch/{arch}/{mode}/traps.S.obj"
        )),
        build_dir.join(format!(
            "kernel/CMakeFiles/kernel.elf.dir/src/arch/{arch}/{archv}/{armv}/machine_asm.S.obj"
        )),
        out_dir.join("kernel.dtb.obj"),
    ]
    .into_iter()
    .map(|x| x.to_str().unwrap().to_string())
    .collect();

    let mut args = vec![
        format!("-r"),
        format!("-c"),
        format!("-s"),
        format!("{}", out_dir.join("libkernel.a").to_str().unwrap()),
    ];

    args.append(&mut obj_files);

    let output = cmd("arm-unknown-linux-gnueabi-ar", args).read()?;

    for line in output.lines() {
        println!("cargo:warning={}", line);
    }

    Ok(())
}

fn redefine_start_sym(out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();

    let args = vec![
        format!("--redefine-sym"),
        format!("_start=_kernel_entry"),
        format!("{}", out_dir.join("libkernel.a").to_str().unwrap()),
    ];

    let output = cmd("arm-unknown-linux-gnueabi-objcopy", args).read()?;

    for line in output.lines() {
        println!("cargo:warning={}", line);
    }

    Ok(())
}

fn prefix_sections(out_dir: impl AsRef<Path>) -> Result<()> {
    let out_dir = out_dir.as_ref();

    let args = vec![
        format!("--prefix-sections"),
        format!(".kernel"),
        format!("{}", out_dir.join("libkernel.a").to_str().unwrap()),
    ];

    let output = cmd("arm-unknown-linux-gnueabi-objcopy", args).read()?;

    for line in output.lines() {
        println!("cargo:warning={}", line);
    }

    Ok(())
}
