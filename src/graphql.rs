use async_graphql::http::GraphiQLSource;
use async_graphql::{Context, EmptySubscription, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::Html;
use axum::routing::get;
use axum::{extract::Extension, response::IntoResponse, routing::post, Router};

use crate::entities::{Devices, InsertionDevice};
use crate::sqlite::SqliteService;

pub async fn serve_graphql(db: SqliteService) {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

struct Query;

#[Object]
impl Query {
    async fn insertion_device(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<InsertionDevice>> {
        let db = ctx.data::<SqliteService>()?;
        let insertion_devices = SqliteService::get_insertion_devices(&db).await.unwrap();
        Ok(insertion_devices)
    }
    async fn all_devices(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Devices>> {
        let db = ctx.data::<SqliteService>()?;
        let all_devices = SqliteService::get_devices(&db).await.unwrap();
        Ok(all_devices)
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn add_insertion_device(
        &self,
        ctx: &Context<'_>,
        poles: i32,
        length: f64,
    ) -> async_graphql::Result<Vec<InsertionDevice>> {
        let db = ctx.data::<SqliteService>()?;
        let results = sqlx::query_as::<_, InsertionDevice>(
            "INSERT INTO insertion_device (poles, length) VALUES ($1, $2)",
        )
        .bind(poles)
        .bind(length)
        .fetch_all(&db.pool)
        .await?;
        Ok(results)
    }
}

async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
