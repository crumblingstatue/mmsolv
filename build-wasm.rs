#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
cmd_lib = "1.9.5"
---

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd_lib::run_cmd! {
        cargo build
            --target=wasm32-unknown-unknown
            --release
            -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort
    }?;
    let target_dir: std::path::PathBuf =
        cmd_lib::run_fun!(cargo metadata | jq -r .target_directory)?.into();
    let target_dir = target_dir.join("wasm32-unknown-unknown/release/graphical-solve.wasm");
    cmd_lib::run_cmd!(wasm-opt -Os --strip-debug $target_dir -o graphical-solve.wasm)?;
    Ok(())
}
