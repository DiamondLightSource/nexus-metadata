mod entities;
mod graphql;
mod sqlite;

const DB_PATH: &str = "sqlite.db";

#[tokio::main]
async fn main() {
    let db = sqlite::SqliteService::connect(DB_PATH).await.unwrap();
    graphql::serve_graphql(db).await;
}
