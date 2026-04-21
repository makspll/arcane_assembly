use crate::commands::*;
use anyhow::{Context, anyhow};
use std::{path::PathBuf, process::Command};

#[derive(clap::Args)]
pub struct ExecArgs {
    #[arg(
        long,
        short,
        help = "the output directory in which to place artifacts if running in release mode, by default 'output'"
    )]
    output: Option<PathBuf>,

    #[arg(
        long,
        short,
        help = "if set will build in --release mode, and run the packaged version"
    )]
    release: Option<bool>,
}

pub fn run_exec(args: ExecArgs) -> Result<(), anyhow::Error> {
    let mut exec_cmd = if args.release.is_none_or(|r| !r) {
        let mut cmd = Command::new("cargo");
        cmd.arg("run");
        cmd
    } else {
        let executable_path = run_package(PackageArgs {
            output: args.output.clone(),
        })
        .with_context(|| "executing release build")?;
        let mut cmd = Command::new(executable_path);
        // otherwise asset paths get messed
        cmd.env_remove("CARGO_MANIFEST_DIR")
            .env_remove("CARGO")
            .env_remove("CARGO_PKG_NAME")
            .env_remove("CARGO_PKG_VERSION");
        cmd
    };

    // need to run as subprocess for current_exe to point to the right place
    let status = exec_cmd
        .spawn()
        .with_context(|| format!("Spawning {:?}", exec_cmd))?
        .wait()
        .with_context(|| format!("Executing {:?}", exec_cmd))?;
    if !status.success() {
        return Err(anyhow!(
            "Non success status {}, in run command: {:?}",
            status,
            exec_cmd
        ));
    }
    Ok(())
}
