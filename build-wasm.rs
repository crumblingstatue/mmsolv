#!/usr/bin/env -S cargo +nightly -Zscript
#![feature(anonymous_pipe)]

use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cargo")
        .args([
            "build",
            "--target=wasm32-unknown-unknown",
            "--release",
            "-Zbuild-std=std,panic_abort",
            "-Zbuild-std-features=panic_immediate_abort",
        ])
        .status()?;
    let (r, w) = std::io::pipe()?;
    Command::new("cargo").arg("metadata").stdout(w).spawn()?;
    let out = Command::new("jq")
        .arg(".target_directory")
        .stdin(r)
        .output()?
        .stdout;
    let target_dir = String::from_utf8(out)?;
    let target_dir = target_dir
        .trim()
        .trim_start_matches('"')
        .trim_end_matches('\"');
    let path = format!("{target_dir}/wasm32-unknown-unknown/release/graphical-solve.wasm");
    Command::new("wasm-opt")
        .args(["-Os", "--strip-debug", &path, "-o", "graphical-solve.wasm"])
        .status()?;
    Ok(())
}
