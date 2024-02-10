mod entity;
mod api;
mod service;
mod router;

use anyhow::Ok;
use dotenvy::dotenv;
use entity::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database, EntityTrait};
use std::env;
use tracing::info;

use crate::entity::user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect("`.env` file not found");
    env::set_var("RUST_LOG", "DEBUG");
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;
    let connect_option = ConnectOptions::new(database_url).to_owned();
    let conn = Database::connect(connect_option).await.unwrap();
    info!("connection: {:?}", conn);

    let p = user::ActiveModel {
        id: ActiveValue::default(),
        username: ActiveValue::Set("Jack".to_owned()),
        password: ActiveValue::Set("123456".to_owned()),
        phone: ActiveValue::Set(Some("12345678910".to_owned())),
    };
    p.insert(&conn).await?;

    let query_res = User::find_by_id(1).one(&conn).await?;
    info!("{:?}", query_res.unwrap());

    Ok(())
}
