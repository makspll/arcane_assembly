use anyhow::{Context, anyhow};
use std::{path::PathBuf, process::Command};

use crate::utils::copy_dir_recursive;

#[derive(clap::Args, Debug)]
pub struct PackageArgs {
    #[arg(
        long,
        short,
        help = "the output directory in which to place artifacts, by default 'output'"
    )]
    pub output: Option<PathBuf>,
}

const EXEC_NAME: &str = "arcane_assembly";

pub fn run_package(args: PackageArgs) -> Result<PathBuf, anyhow::Error> {
    let mut build_cmd = Command::new("cargo");

    let output_dir = args.output.unwrap_or(PathBuf::from("output"));

    build_cmd
        .arg("build")
        .arg("--no-default-features")
        .arg("--release");

    let status = build_cmd.status()?;
    if !status.success() {
        return Err(anyhow!(
            "Non success status: {}, when running cmd: {:?}",
            status,
            build_cmd
        ));
    }

    // move built executable into output dir
    let target_dir = PathBuf::from("target");
    let exec_directory = target_dir.join("release");
    let exec_filename = format!("{EXEC_NAME}{}", std::env::consts::EXE_SUFFIX);
    let exec_path = exec_directory.join(&exec_filename);
    let output_exec_path = output_dir.join(&exec_filename);

    if !(output_dir.is_dir()) {
        return Err(anyhow!(
            "output directory: {:?} is not a directory",
            output_dir
        ));
    }

    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("while creating all directories for: {output_dir:?}"))?;
    std::fs::copy(&exec_path, &output_exec_path).with_context(|| {
        format!("while copying binary from: {exec_path:?} to {output_exec_path:?}")
    })?;

    // move assets into directory next to exec
    let asset_dir = PathBuf::from("assets");
    let output_asset_dir = output_dir.join("assets");
    copy_dir_recursive(&asset_dir, &output_asset_dir)
        .with_context(|| "copying assets".to_string())?;

    Ok(output_exec_path)
}
