#![feature(fs_try_exists)]

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use duct::*;

fn main() -> Result<()> {
    let out_dir: PathBuf = env::var("OUT_DIR")?.into();

    let (sel4_dir, build_dir, is_custom_build): (PathBuf, PathBuf, bool) =
        if let (Ok(sel4_dir), Ok(build_dir)) = (env::var("SEL4_DIR"), env::var("SEL4_BUILD_DIR")) {
            println!("cargo:rerun-if-env-changed=SEL4_DIR");
            println!("cargo:rerun-if-env-changed=SEL4_BUILD_DIR");
            (sel4_dir.into(), build_dir.into(), true)
        } else {
            (
                env::current_dir()?.join("sel4"),
                out_dir.join("build"),
                false,
            )
        };

    if !sel4_dir.is_absolute() {
        return Err(anyhow!("SEL4_DIR must be an absolute path!"));
    }

    if !build_dir.is_absolute() {
        return Err(anyhow!("SEL4_BUILD_DIR must be an absolute path!"));
    }

    // todo: for custom (external) builds verify given config matches given build artifacts (<sel4 build dir>/kernel/gen_config/gen_config.json)

    if !is_custom_build {
        make_internal_build(&sel4_dir, &build_dir)?;
    }

    println!(
        "cargo:DIR={}",
        sel4_dir.to_str().ok_or(anyhow!("Invalid path"))?
    );

    println!(
        "cargo:BUILD_DIR={}",
        build_dir.to_str().ok_or(anyhow!("Invalid path"))?
    );

    if is_custom_build {
        println!("cargo:rerun-if-changed={}", sel4_dir.to_str().unwrap());
        println!("cargo:rerun-if-changed={}", build_dir.to_str().unwrap());
    }

    Ok(())
}

fn make_internal_build(_sel4_dir: impl AsRef<Path>, build_dir: impl AsRef<Path>) -> Result<()> {
    let toolchain_config = ToolchainConfig::get();
    let sel4_config = SeL4Config::get();

    // if fs::try_exists(&build_dir)? {
    //     fs::remove_dir_all(&build_dir)?;
    // }

    // fs::create_dir(&build_dir)?;

    // cmake_config(toolchain_config, sel4_config, &build_dir)?;
    // ninja_build(&build_dir)?;

    if !fs::try_exists(&build_dir)? {
        fs::create_dir(&build_dir)?;
        cmake_config(toolchain_config, sel4_config, &build_dir)?;
    }

    ninja_build(&build_dir)?;

    Ok(())
}

#[derive(Debug)]
struct ToolchainConfig {
    cross_compiler_prefix: String,
    arch_mode: ArchMode,
    toolchain_file: PathBuf,
}

#[derive(Debug, Clone, Copy)]
enum ArchMode {
    Aarch32,
    // Aarch64,
    // RiscV32,
    // RiscV64,
}

impl ToolchainConfig {
    fn get() -> Self {
        Self {
            cross_compiler_prefix: Self::cross_compiler_prefix(),
            arch_mode: Self::arch_mode(),
            toolchain_file: "kernel/gcc.cmake".into(),
        }
    }

    fn cross_compiler_prefix() -> String {
        println!("cargo:rerun-if-env-changed=SEL4_CROSS_COMPILER_PREFIX");
        let cross_compiler_prefix = env::var("SEL4_CROSS_COMPILER_PREFIX").ok();

        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let compiler_prefixes = HashMap::from([
            ("arm".to_string(), "arm-unknown-linux-gnueabi-".to_string()),
            // (
            //     "aarch64".to_string(),
            //     "aarch64-unknown-linux-gnu-".to_string(),
            // ),
            // ("riscv32".to_string(), "riscv32-unknown-linux-gnu-".to_string()),
            // ("riscv64".to_string(), "riscv64-unknown-linux-gnu-".to_string()),
        ]);

        cross_compiler_prefix
            .or_else(|| compiler_prefixes.get(&target_arch).cloned())
            .expect("Invalid target arch!")
    }

    fn arch_mode() -> ArchMode {
        println!("cargo:rerun-if-env-changed=SEL4_ARCH_MODE");
        let arch_mode = env::var("SEL4_ARCH_MODE").ok().map(|arch_mode| {
            match arch_mode.to_lowercase().trim() {
                "aarch32" => ArchMode::Aarch32,
                // "aarch64" => ArchMode::Aarch64,
                // "riscv32" => ArchMode::RiscV32,
                // "riscv64" => ArchMode::RiscV64,
                _ => panic!("Invalid arch mode (AARCH32, ..)!"),
            }
        });

        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let arch_modes = HashMap::from([
            ("arm".to_string(), ArchMode::Aarch32),
            // ("aarch64".to_string(), ArchMode::Aarch64),
            // ("riscv32".to_string(), ArchMode::RiscV32),
            // ("riscv64".to_string(), ArchMode::RiscV64),
        ]);

        arch_mode
            .or_else(|| arch_modes.get(&target_arch).cloned())
            .expect("Invalid target arch!")
    }

    fn cmake_args(&self) -> Vec<String> {
        vec![
            format!("-G {}", "Ninja"),
            format!("-DCROSS_COMPILER_PREFIX={}", self.cross_compiler_prefix),
            format!(
                "-D{}=TRUE",
                match self.arch_mode {
                    ArchMode::Aarch32 => "AARCH32",
                    // ArchMode::Aarch64 => "AARCH64",
                    // ArchMode::RiscV32 => "RISCV32",
                    // ArchMode::RiscV64 => "RISCV64",
                }
            ),
            format!(
                "-DCMAKE_TOOLCHAIN_FILE={}",
                self.toolchain_file.to_str().unwrap()
            ),
        ]
    }
}

#[derive(Debug)]
struct SeL4Config {
    debug: bool,
    platform: String,
    mcs: bool,
    smp: bool,
    dangerous_code_injection: bool,
    devicetree: Option<PathBuf>,
    devicetree_overlay: Option<PathBuf>,
}

impl SeL4Config {
    fn get() -> Self {
        SeL4Config {
            debug: Self::debug(),
            platform: Self::platform(),
            mcs: Self::mcs(),
            smp: Self::smp(),
            dangerous_code_injection: Self::dangerous_code_injection(),
            devicetree: Self::devicetree(),
            devicetree_overlay: Self::devicetree_overlay(),
        }
    }

    fn debug() -> bool {
        match () {
            #[cfg(debug_assertions)]
            () => true,
            #[cfg(not(debug_assertions))]
            () => false,
        }
    }

    fn platform() -> String {
        match () {
            #[cfg(feature = "zynq7000")]
            () => "zynq7000".to_string(),
            #[cfg(feature = "zcu102")]
            () => "zcu102".to_string(),
            // #[cfg(feature = "spike")]
            // () => "spike".to_string(),
            // #[cfg(feature = "hifive")]
            // () => "hifive".to_string(),
        }
    }

    fn mcs() -> bool {
        match () {
            #[cfg(feature = "mcs")]
            () => true,
            #[cfg(not(feature = "mcs"))]
            () => false,
        }
    }

    fn smp() -> bool {
        match () {
            #[cfg(feature = "smp")]
            () => true,
            #[cfg(not(feature = "smp"))]
            () => false,
        }
    }

    fn dangerous_code_injection() -> bool {
        match () {
            #[cfg(feature = "dangerous-code-injection")]
            () => true,
            #[cfg(not(feature = "dangerous-code-injection"))]
            () => false,
        }
    }

    fn devicetree() -> Option<PathBuf> {
        println!("cargo:rerun-if-env-changed=SEL4_DEVICETREE");
        env::var("SEL4_DEVICETREE").ok().map(String::into)
    }

    fn devicetree_overlay() -> Option<PathBuf> {
        println!("cargo:rerun-if-env-changed=SEL4_DEVICETREE_OVERLAY");
        env::var("SEL4_DEVICETREE_OVERLAY").ok().map(String::into)
    }

    fn cmake_args(&self) -> Vec<String> {
        vec![
            format!("-DRELEASE={}", if self.debug { "OFF" } else { "ON" }),
            format!("-DPLATFORM={}", &self.platform),
            format!("-DKernelIsMCS={}", if self.mcs { "ON" } else { "OFF" }),
            format!("-DSMP={}", if self.smp { "ON" } else { "OFF" }),
            format!("-DKernelMaxNumNodes={}", if self.smp { 4 } else { 1 }),
            format!(
                "-DKernelDangerousCodeInjection={}",
                if self.dangerous_code_injection {
                    "ON"
                } else {
                    "OFF"
                }
            ),
            format!(
                "-DKernelCustomDTS={}",
                if let Some(dts) = self.devicetree.as_ref() {
                    dts.to_str().unwrap_or("")
                } else {
                    ""
                }
            ),
            format!(
                "-DKernelCustomDTSOverlay={}",
                if let Some(dts) = self.devicetree_overlay.as_ref() {
                    dts.to_str().unwrap_or("")
                } else {
                    ""
                }
            ),
            format!("-DVERIFICATION={}", "OFF"),
            format!("-DLibSel4FunctionAttributes={}", "public"),
        ]
    }
}

fn cmake_config(
    toolchain: ToolchainConfig,
    sel4: SeL4Config,
    build_dir: impl AsRef<Path>,
) -> Result<()> {
    let build_dir = build_dir.as_ref();

    let mut args = toolchain.cmake_args();
    args.append(&mut sel4.cmake_args());
    args.append(&mut vec![
        format!("-S {}", "sel4"),
        format!("-B {}", build_dir.to_str().unwrap()),
    ]);

    cmd("cmake", args).run()?;

    Ok(())
}

fn ninja_build(build_dir: impl AsRef<Path>) -> Result<()> {
    let build_dir = build_dir.as_ref();

    cmd!("ninja", "-C", build_dir).run()?;

    Ok(())
}
