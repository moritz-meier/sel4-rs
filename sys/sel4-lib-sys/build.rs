use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use bindgen::Builder;

fn main() -> Result<()> {
    let out_dir: PathBuf = env::var("OUT_DIR")?.into();
    let sel4_dir: PathBuf = env::var("DEP_SEL4_DIR")?.into();
    let build_dir: PathBuf = env::var("DEP_SEL4_BUILD_DIR")?.into();

    // println!("cargo:warning=SEL4_DIR={:?}", sel4_dir);
    // println!("cargo:warning=BUILD_DIR={:?}", build_dir);

    copy_artifact(&build_dir, &out_dir)?;

    let inc_dirs: Vec<PathBuf> = get_include_dirs(&sel4_dir, &build_dir);
    generate_bindings(&sel4_dir, &out_dir, inc_dirs.into_iter())?;

    println!("cargo:rustc-link-search={}", out_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=sel4");

    Ok(())
}

fn copy_artifact(build_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();
    let out_dir = out_dir.as_ref();

    let libsel4 = build_dir.join("libsel4/libsel4.a");
    fs::copy(libsel4, out_dir.join("libsel4.a"))?;

    Ok(())
}

fn get_include_dirs(sel4_dir: impl AsRef<Path>, build_dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let build_dir: PathBuf = build_dir.as_ref().into();
    let sel4_dir: PathBuf = sel4_dir.as_ref().into();

    let (arch, sel4_arch, sel4_plat, mode) = match () {
        #[cfg(feature = "stm32mp1")]
        () => ("arm", "aarch32", "stm32mp1", "32"),
        #[cfg(feature = "zynq7000")]
        () => ("arm", "aarch32", "zynq7000", "32"),
    };

    vec![
        sel4_dir.join("kernel/libsel4/include"),
        sel4_dir.join(format!("kernel/libsel4/arch_include/{arch}")),
        sel4_dir.join(format!("kernel/libsel4/sel4_arch_include/{sel4_arch}")),
        sel4_dir.join(format!("kernel/libsel4/sel4_plat_include/{sel4_plat}")),
        sel4_dir.join(format!("kernel/libsel4/mode_include/{mode}")),
        build_dir.join("kernel/gen_config"),
        build_dir.join("libsel4/autoconf"),
        build_dir.join("libsel4/gen_config"),
        build_dir.join("libsel4/include"),
        build_dir.join(format!("libsel4/arch_include/{arch}")),
        build_dir.join(format!("libsel4/sel4_arch_include/{sel4_arch}")),
    ]
}

fn generate_bindings(
    sel4_dir: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
    include_dirs: impl Iterator<Item = impl AsRef<Path>>,
) -> Result<()> {
    let sel4_dir = sel4_dir.as_ref();
    let out_dir = out_dir.as_ref();

    let sel4_header = "kernel/libsel4/include/sel4/sel4.h";

    let bindings = Builder::default()
        .header(sel4_dir.join(sel4_header).to_str().unwrap())
        .clang_arg("--target=arm-linux-gnueabi")
        .clang_args(
            include_dirs
                .into_iter()
                .map(|dir| format!("-I{}", dir.as_ref().to_str().unwrap())),
        )
        .clang_arg("-D KernelDangerousCodeInjection OFF")
        .use_core()
        .layout_tests(false)
        .generate()?;

    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
