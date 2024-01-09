use clap::Args;
use fake::{Fake, Faker};
use sea_orm::Database;

use crate::conf::Conf;
use crate::dto::article::ArticleRequest;
use crate::dto::category::CategoryRequest;
use crate::dto::tag::TagRequest;
use crate::dto::user::NewUser;
use crate::repos::{article, category, tag, user};

#[derive(Debug, Args)]
pub struct Cmd {}

pub fn handle(_cmd: &Cmd, conf: &Conf) -> anyhow::Result<()> {
    println!("Fake some data for test");

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let db_url = conf.database.url.clone().unwrap_or("".to_string());
            let conn = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            let user_data: NewUser = Faker.fake();
            let user_id = user::create(&conn, &user_data)
                .await
                .expect("Create user failed");

            for _i in 0..30 {
                let tag_data: TagRequest = Faker.fake();
                let t = tag::check_name_exist(&conn, &tag_data.name)
                    .await
                    .unwrap_or(None);
                if t.is_some() {
                    continue;
                }

                tag::create(&conn, &tag_data)
                    .await
                    .expect("Create tag failed");
            }

            let mut category_id = 1;
            for _i in 0..10 {
                let category_data: CategoryRequest = Faker.fake();
                let c = category::check_name_exist(&conn, &category_data.name)
                    .await
                    .unwrap_or(None);
                if c.is_some() {
                    continue;
                }

                category_id = category::create(&conn, &category_data)
                    .await
                    .expect("Create category failed");
            }

            for _i in 0..30 {
                let mut article_data: ArticleRequest = Faker.fake();
                article_data.category_id = Some(category_id);
                article::create(&conn, user_id, &article_data)
                    .await
                    .expect("Create article failed");
            }
        });

    Ok(())
}
