use async_graphql::http::GraphiQLSource;
use async_graphql::{
    EmptyMutation, EmptySubscription, Enum, Object, OutputType, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::Html;
use axum::routing::get;
use axum::{extract::Extension, response::IntoResponse, routing::post, Router};

pub async fn serve_graphql() -> () {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql_handler))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// NeXus definitions

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum InsertionDeviceType {
    Undulator,
    Wiggler,
}

type Length = f64;
type Angle = f64;
type Power = f64;
type Energy = f64;

#[derive(SimpleObject)]
pub struct NxInsertionDevice {
    default: String,
    id_type: InsertionDeviceType, // TODO: Figure out how to alias this
    gap: Length,
    taper: Angle,
    phase: Angle,
    poles: i32,
    magnetic_wavelength: Length,
    k: f64,
    length: Length,
    power: Power,
    energy: Energy,
    bandwidth: Energy,
    harmonic: i32,
    depends_on: String,
}

#[derive(SimpleObject)]
pub struct Devices<T: OutputType> {
    devices: Vec<T>,
}

struct Query;

#[Object]
impl Query {
    async fn insertion_device(&self) -> Devices<NxInsertionDevice> {
        Devices { devices: vec![] }
    }
    async fn hello_foo(&self, foo: String) -> String {
        format!("hello {foo}")
    }
}

async fn graphql_handler(
    schema: Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
