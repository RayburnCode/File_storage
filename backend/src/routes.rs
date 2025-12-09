use axum::{
    routing::{get, post},
    Router,
};
use crate::{user, save_file};
use tower_http::cors::{CorsLayer, Any};

pub async fn run_server() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(user::root))
        // `POST /users` goes to `create_user`
        .route("/users", post(user::create_user))
        .route("/save", post(save_file::save_file))
        .layer(cors);


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
