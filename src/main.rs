#![forbid(unsafe_code)]

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, routing::post, Router};

struct Query;

#[Object]
impl Query {
    async fn person(&self) -> Person {
        Person {
            id: 1,
            first_name: "foo".to_string(),
            last_name: "bar".to_string(),
            preferred_name: None,
        }
    }
}

#[derive(SimpleObject)]
struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    preferred_name: Option<String>,
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn graphql_handler(
    schema: Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
