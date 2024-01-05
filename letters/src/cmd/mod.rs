mod create_admin;
mod hello;
mod migrate;
mod serve;

use clap::{Parser, Subcommand};

use crate::conf::Conf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cmd {
    /// Configuration file location
    #[arg(
        short,
        long,
        value_name = "config",
        default_value = "config/default.toml"
    )]
    pub config: Option<String>,

    #[command(subcommand)]
    pub subcmd: Option<SubCmd>,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    /// Hello World!
    Hello(hello::Cmd),
    /// Start HTTP server
    Serve(serve::Cmd),
    /// Run database migrations
    Migrate(migrate::Cmd),
    // Create the default admin user
    CreateAdmin(create_admin::Cmd),
}

pub fn setup() -> anyhow::Result<Cmd> {
    let cmd = Cmd::parse();
    Ok(cmd)
}

pub fn handle(cmd: &Cmd, conf: &Conf) -> anyhow::Result<()> {
    match &cmd.subcmd {
        Some(SubCmd::Serve(subcmd)) => {
            serve::handle(subcmd, conf)?;
        }
        Some(SubCmd::Hello(subcmd)) => {
            hello::handle(subcmd, conf)?;
        }
        Some(SubCmd::Migrate(subcmd)) => {
            migrate::handle(subcmd, conf)?;
        }
        Some(SubCmd::CreateAdmin(subcmd)) => {
            create_admin::handle(subcmd, conf)?;
        }
        None => todo!(),
    }

    Ok(())
}
