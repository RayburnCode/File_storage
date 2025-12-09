mod user;
mod routes;
mod save_file; 

#[tokio::main]
async fn main() {
    // Run the axum server
    routes::run_server().await;
}

