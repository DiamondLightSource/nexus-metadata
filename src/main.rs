mod graphql;

#[tokio::main]
async fn main() {
    graphql::serve_graphql().await;
}
