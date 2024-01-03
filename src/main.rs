use dotenvy::dotenv;
use letters::{cmd, conf::Conf, log};

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    log::setup();

    let cli = cmd::setup()?;

    let location = cli.config.clone().unwrap_or("".to_string());
    let conf = Conf::new(&location, "LETTERS")?;

    cmd::handle(&cli, &conf)?;

    Ok(())
}
