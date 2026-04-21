use clap::Parser;
use xtask_lib::{cli::*, commands::CmdRunner};

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let runner = CmdRunner;
    runner.run(args)
}
