use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;
use deadpool_postgres::tokio_postgres::Row;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "logs")]
pub struct LogReport {
    pub id: i32,
    pub log_type: String,
    pub severity: String,
    pub date: String,
    pub username: String,
    pub actual_log: String,
}

impl From<&Row> for LogReport {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            log_type: row.get("type"),
            severity: row.get("severity"),
            date: row.get("date"),
            username: row.get("username"),
            actual_log: row.get("actualLog"),
        }
    }
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}


#[derive(Deserialize)]
pub struct AddLog {
    pub log_type: String,
    pub severity: String,
    pub date: String,
    pub log: String
}