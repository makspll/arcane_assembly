use crate::commands::*;

#[derive(clap::Parser)]
#[command(about = "CLI utils for the game")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(clap::Subcommand)]
pub enum CliCommand {
    #[command(
        about = "Package the executable in a way that can be distributed for the current OS",
        name = "package"
    )]
    Package(PackageArgs),
    #[command(
        about = "Buld and execute the binary, packaging first if running in release mode",
        name = "run"
    )]
    Execute(ExecArgs),
}
