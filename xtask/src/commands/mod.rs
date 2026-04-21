mod exec;
mod package;

use crate::cli::{Cli, CliCommand};

pub struct CmdRunner;

impl CmdRunner {
    pub fn run(&self, cmd: Cli) -> Result<(), anyhow::Error> {
        match cmd.command {
            CliCommand::Package(args) => run_package(args).map(|_| ()),
            CliCommand::Execute(args) => run_exec(args).map(|_| ()),
        }
    }
}

pub use {exec::*, package::*};
