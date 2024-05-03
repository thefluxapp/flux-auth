use async_graphql::{EmptyMutation, EmptySubscription, Object, Result, Schema};
use async_graphql_axum::GraphQL;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new().route_service("/gql", GraphQL::new(schema));

    axum::serve(TcpListener::bind("0.0.0.0:3000").await.unwrap(), app)
        .await
        .unwrap();
}

struct Query;

#[Object]
impl Query {
    async fn join(&self) -> Result<bool> {
        Ok(true)
    }
}
