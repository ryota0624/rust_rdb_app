use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use server::Schema;
use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = async_graphql::Schema::from(Schema::new("server".to_string()));
    let graphql = GraphQL::new(schema);
    let app = Router::new().route("/", get(graphiql).post_service(graphql));
    println!("GraphiQL IDE: http://localhost:8000");
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
