use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "turbo")]
#[command(about="project creation and onboarding tool", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub enum ProjectTypes {
    Web,
    Mobile,
    Service,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Version,
    #[command(arg_required_else_help = true)]
    New {
        #[arg(required = true)]
        name: String,
    },
}
