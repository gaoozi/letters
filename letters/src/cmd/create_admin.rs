use clap::Args;

use crate::{conf::Conf, dto::user::NewUser, repos::user};
use sea_orm::Database;

#[derive(Debug, Args)]
pub struct Cmd {
    // Email for admin user
    #[arg(short, long, value_name = "EMAIL", default_value = "test@123.com")]
    email: Option<String>,
    /// Password for admin user
    #[arg(short, long, value_name = "PASSWORD", default_value = "Pa$$wd123")]
    password: Option<String>,
}

pub fn handle(cmd: &Cmd, conf: &Conf) -> anyhow::Result<()> {
    let password = cmd.password.clone().unwrap();
    let email = cmd.email.clone().unwrap();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let db_url = conf.database.url.clone().unwrap_or("".to_string());
            let conn = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            if user::check_username_exist(&conn, "admin").await? {
                println!("Admin user already exists");
                return Ok::<(), anyhow::Error>(());
            }

            user::create(
                &conn,
                &NewUser {
                    username: "admin".to_string(),
                    email,
                    password,
                },
            )
            .await?;

            println!("Admin user created.");

            Ok(())
        })?;

    Ok(())
}
