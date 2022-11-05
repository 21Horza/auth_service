use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use std::sync::Arc;
use std::convert::Infallible;
use warp::Filter;
use std::fs;
use log::error;

const SQL_FILE: &str = "src/sql/db.sql";

const PG_HOST: &str = "localhost";
const PG_DB: &str = "postgres";
const PG_USER: &str = "postgres";
const PG_PWD: &str = "rootroot";
const PG_MAX_CON: u32 = 5;

pub type Db = Pool<Postgres>;

// new DB pool
async fn new_db_pool(
    user: &str,
    pwd: &str,
    host: &str,
    db: &str,
    max_conn: u32
) -> Result<Db, sqlx::Error> {
    let conn_string: String = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_conn)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&conn_string)
        .await
}

// sql exec file
async fn sql_exec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let content = fs::read_to_string(file).map_err(|e| {
        error!("Reading file {} failed with err: {}", file, e);
        e
    })?;

    // split the content
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        match sqlx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(e) => error!("Sql file {} err: {:?}", file, e)
        }
    }
    Ok(())
}

// init db
pub async fn init_db() -> Result<Db, sqlx::Error> {
    {
        let db_pool = new_db_pool(PG_USER, PG_PWD, PG_HOST, PG_DB, 1).await?;
        sql_exec(&db_pool, SQL_FILE).await?;
    }
    new_db_pool(PG_USER, PG_PWD, PG_HOST, PG_DB, PG_MAX_CON).await
}

// helper function
pub fn with_db_pool(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}