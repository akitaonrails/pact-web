use std::sync::{Arc, Mutex};

use axum::routing::{get, post};
use axum::Router;
use pact_runtime::prelude::InMemoryStore;

mod generated;
mod handlers;
mod html;

use generated::user_service::User;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<InMemoryStore<User>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        store: Arc::new(Mutex::new(InMemoryStore::new())),
    };

    let app = Router::new()
        // HTML routes
        .route("/", get(handlers::list_users))
        .route("/users/new", get(handlers::new_user_form))
        .route("/users", post(handlers::create_user_handler))
        .route("/users/{id}", get(handlers::show_user))
        .route("/users/{id}/delete", post(handlers::delete_user))
        // JSON API routes
        .route("/api/users", get(handlers::api_list_users))
        .route("/api/users", post(handlers::api_create_user))
        .route("/api/users/{id}", get(handlers::api_get_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    eprintln!("Pact Web listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
