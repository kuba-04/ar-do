use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ardo", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Start/resume time entry")]
    Start {
        #[arg(help = "what are you starting")]
        comment: Option<String>,
    },
    #[command(about = "Stop/pause time entry")]
    Stop {},
    #[command(about = "Check status")]
    Status {},
    #[command(about = "Account Info")]
    Info {},
}
