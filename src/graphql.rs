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
    default: Option<String>,
    r#type: Option<InsertionDeviceType>,
    gap: Option<Length>,
    taper: Option<Angle>,
    phase: Option<Angle>,
    poles: Option<i32>,
    magnetic_wavelength: Option<Length>,
    k: Option<f64>,
    length: Option<Length>,
    power: Option<Power>,
    energy: Option<Energy>,
    bandwidth: Option<Energy>,
    harmonic: Option<i32>,
    depends_on: Option<String>,
}

#[derive(SimpleObject)]
pub struct Devices<T: OutputType> {
    devices: Vec<T>,
}

struct Query;

#[Object]
impl Query {
    async fn insertion_device(&self) -> Devices<NxInsertionDevice> {
        let device = NxInsertionDevice {
            default: None,
            r#type: None,
            gap: None,
            taper: None,
            phase: None,
            poles: Some(2),
            magnetic_wavelength: None,
            k: None,
            length: Some(4.0),
            power: None,
            energy: None,
            bandwidth: None,
            harmonic: None,
            depends_on: None,
        };
        Devices {
            devices: vec![device],
        }
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
