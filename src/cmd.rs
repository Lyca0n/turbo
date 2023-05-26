use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[command(name = "turbo")]
#[command(about="project creation and onboarding tool", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Version,
    #[command(arg_required_else_help = true)]
    New {
        #[arg(required = true)]
        name: String,
    },
    Mold(MoldArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MoldArgs {
    #[command(subcommand)]
    pub command: Option<MoldCommands>,
    pub name:String,
}

#[derive(Debug, Subcommand)]
pub enum MoldCommands {
    List,
    Diff{vars:String}
}