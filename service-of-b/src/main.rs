use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    println!("Runing the entry file of the service B");
    // initialize port number
    let port = "127.0.0.1:4000";
    // let port = 4000;

    let app = Router::new()
        .route("/", get(root))
        .route("/ping", get(get_ping));

    println!("Service B running on http://localhost:4000");

    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    println!("Calling from the root file");
    "Welcome to Service B"
}

async fn get_ping() -> &'static str {
    println!("Hello from service B");
    "Welcome to service B"
}