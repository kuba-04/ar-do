use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(name = "ardo", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Start/resume time entry")]
    Start {},
    #[command(about = "Stop/pause time entry")]
    Stop {},
    #[command(about = "Check status")]
    Status {},
    #[command(about = "Account Info")]
    Info {}

}