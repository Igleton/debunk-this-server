use axum::Router;

mod api;
mod core;
mod state;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api", api::api::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
