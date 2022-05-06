use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod crawl_queue;
pub mod fetch_history;
pub mod indexed_document;
pub mod link;
pub mod resource_rule;

use shared::config::Config;

pub async fn create_connection(is_test: bool) -> anyhow::Result<DatabaseConnection> {
    let db_uri: String = if is_test {
        "sqlite::memory:".to_string()
    } else {
        format!(
            "sqlite://{}?mode=rwc",
            Config::data_dir().join("db.sqlite").to_str().unwrap()
        )
    };

    // See https://www.sea-ql.org/SeaORM/docs/install-and-config/connection
    // for more connection options
    let mut opt = ConnectOptions::new(db_uri);
    opt.max_connections(5).sqlx_logging(false);

    Ok(Database::connect(opt).await?)
}

#[cfg(test)]
mod test {
    use crate::models::create_connection;

    #[tokio::test]
    async fn test_create_connection() {
        let res = create_connection(true).await;
        assert!(res.is_ok());
    }
}
