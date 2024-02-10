use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};
use tokio::sync::OnceCell;

pub async fn database_connection() -> &'static DatabaseConnection {
    static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();
    DB_CONN
        .get_or_init(|| async {
            let database_url = env::var("DATABASE_URL").unwrap();
            let mut opt = ConnectOptions::new(database_url);
            opt.max_connections(100)
                .connect_timeout(Duration::from_secs(8))
                .acquire_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .max_lifetime(Duration::from_secs(8));

            Database::connect(opt).await.unwrap()
        })
        .await
}

#[cfg(test)]
mod test {
    use std::env;

    use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

    use crate::{
        db::database_connection,
        entity::{prelude::*, user},
    };

    #[tokio::test]
    async fn test_database_connection() {
        env::set_var("DATABASE_URL", "sqlite:./.local/mydb.sqlite?mode=rwc");
        let p = user::ActiveModel {
            id: ActiveValue::default(),
            username: ActiveValue::Set("Jack".to_owned()),
            password: ActiveValue::Set("123456".to_owned()),
            phone: ActiveValue::Set(Some("12345678910".to_owned())),
        };
        let conn = database_connection().await;
        p.insert(conn).await.unwrap();

        let query_res = User::find().all(conn).await.unwrap();
        println!("{:?}", query_res);
    }
}
