use clap::Args;

use crate::conf::Conf;

#[derive(Debug, Args)]
pub struct Cmd {}

pub fn handle(_cmd: &Cmd, _conf: &Conf) -> anyhow::Result<()> {
    println!("Hello World!");
    Ok(())
}
