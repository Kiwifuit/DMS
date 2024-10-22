use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

async fn index() -> &'static str {
  "Hello world!"
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
  let app = Router::new().route("/", get(index));

  let server = TcpListener::bind("0.0.0.0:3030").await.unwrap();

  axum::serve(server, app).await.unwrap();
}
