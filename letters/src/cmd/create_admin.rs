use clap::Args;

use crate::{conf::Conf, utils::hash::generate_hash};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, Database, EntityTrait, QueryFilter};

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

            // TODO!
            let admins = entity::user::Entity::find()
                .filter(entity::user::Column::Username.eq("admin"))
                .all(&conn)
                .await?;

            if !admins.is_empty() {
                println!("Admin user already exists");
                return Ok::<(), anyhow::Error>(());
            }

            let admin_model = entity::user::ActiveModel {
                username: ActiveValue::Set("admin".to_owned()),
                email: ActiveValue::Set(email),
                password_hash: ActiveValue::Set(generate_hash(&password)?),
                ..Default::default()
            };

            if let Ok(_admin) = admin_model.save(&conn).await {
                println!("Admin user created.");
            } else {
                println!("Failed to create admin user");
            }

            Ok(())
        })?;

    Ok(())
}
