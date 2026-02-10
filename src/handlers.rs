use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use serde::Deserialize;

use crate::html::{html_alert, html_form, html_page, html_table};
use crate::AppState;
use crate::generated::user_service::*;
use pact_runtime::prelude::*;

#[derive(Deserialize)]
pub struct CreateUserForm {
    pub name: String,
    pub email: String,
}

// ─── HTML Handlers ───

pub async fn list_users(State(state): State<AppState>) -> Html<String> {
    let store = state.store.lock().unwrap();
    let users = store.list_all();

    let rows: Vec<Vec<String>> = users
        .iter()
        .map(|u| {
            vec![
                format!(
                    r#"<a href="/users/{}" class="text-indigo-600 hover:text-indigo-800">{}</a>"#,
                    u.id,
                    &u.id.to_string()[..8]
                ),
                u.name.clone(),
                u.email.clone(),
                format!(
                    r#"<form method="POST" action="/users/{}/delete" class="inline">
                        <button type="submit" class="text-red-600 hover:text-red-800 text-sm">Delete</button>
                    </form>"#,
                    u.id
                ),
            ]
        })
        .collect();

    let body = if users.is_empty() {
        format!(
            r#"<h1 class="text-2xl font-bold mb-6">Users</h1>
            <p class="text-gray-500">No users yet. <a href="/users/new" class="text-indigo-600 hover:underline">Create one</a>.</p>"#
        )
    } else {
        format!(
            r#"<h1 class="text-2xl font-bold mb-6">Users ({})</h1>{}"#,
            users.len(),
            html_table(&["ID", "Name", "Email", "Actions"], &rows)
        )
    };

    Html(html_page("Users", &body))
}

pub async fn new_user_form() -> Html<String> {
    let body = format!(
        r#"<h1 class="text-2xl font-bold mb-6">Create User</h1>{}"#,
        html_form(
            "/users",
            &[("name", "Name", "text"), ("email", "Email", "email")]
        )
    );
    Html(html_page("New User", &body))
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateUserForm>,
) -> impl IntoResponse {
    let input = CreateUserInput {
        name: form.name,
        email: form.email,
    };

    let mut store = state.store.lock().unwrap();
    let result = create_user(&mut *store, input);

    match result {
        CreateUserResult::Ok(user) => {
            Redirect::to(&format!("/users/{}?created=1", user.id)).into_response()
        }
        CreateUserResult::ValidationFailed(errors) => {
            let error_msgs: Vec<String> = errors.iter().map(|e| format!("{}: {}", e.field, e.message)).collect();
            let body = format!(
                r#"<h1 class="text-2xl font-bold mb-6">Create User</h1>{}
                {}"#,
                html_alert("error", &error_msgs.join(", ")),
                html_form(
                    "/users",
                    &[("name", "Name", "text"), ("email", "Email", "email")]
                )
            );
            Html(html_page("New User", &body)).into_response()
        }
        CreateUserResult::DuplicateEmail { email } => {
            let body = format!(
                r#"<h1 class="text-2xl font-bold mb-6">Create User</h1>{}
                {}"#,
                html_alert("error", &format!("Email '{}' is already taken", email)),
                html_form(
                    "/users",
                    &[("name", "Name", "text"), ("email", "Email", "email")]
                )
            );
            Html(html_page("New User", &body)).into_response()
        }
    }
}

pub async fn show_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    let result = get_user_by_id(&*store, &id);

    match result {
        GetUserByIdResult::Ok(user) => {
            let body = format!(
                r#"<h1 class="text-2xl font-bold mb-6">User Details</h1>
                <div class="bg-white shadow rounded-lg p-6">
                    <dl class="grid grid-cols-2 gap-4">
                        <dt class="text-sm font-medium text-gray-500">ID</dt>
                        <dd class="text-sm text-gray-900">{}</dd>
                        <dt class="text-sm font-medium text-gray-500">Name</dt>
                        <dd class="text-sm text-gray-900">{}</dd>
                        <dt class="text-sm font-medium text-gray-500">Email</dt>
                        <dd class="text-sm text-gray-900">{}</dd>
                    </dl>
                    <div class="mt-6 flex space-x-4">
                        <a href="/" class="text-indigo-600 hover:underline">Back to list</a>
                        <form method="POST" action="/users/{}/delete" class="inline">
                            <button type="submit" class="text-red-600 hover:underline">Delete</button>
                        </form>
                    </div>
                </div>"#,
                user.id, user.name, user.email, user.id
            );
            Html(html_page("User Details", &body)).into_response()
        }
        GetUserByIdResult::NotFound { id } => {
            let body = format!(
                r#"<h1 class="text-2xl font-bold mb-6">User Not Found</h1>{}
                <a href="/" class="text-indigo-600 hover:underline">Back to list</a>"#,
                html_alert("error", &format!("No user found with ID: {}", id))
            );
            (
                StatusCode::NOT_FOUND,
                Html(html_page("Not Found", &body)),
            )
                .into_response()
        }
        GetUserByIdResult::InvalidId { id } => {
            let body = format!(
                r#"<h1 class="text-2xl font-bold mb-6">Invalid ID</h1>{}
                <a href="/" class="text-indigo-600 hover:underline">Back to list</a>"#,
                html_alert("error", &format!("'{}' is not a valid UUID", id))
            );
            (
                StatusCode::BAD_REQUEST,
                Html(html_page("Invalid ID", &body)),
            )
                .into_response()
        }
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Ok(uuid) = id.parse::<uuid::Uuid>() {
        let mut store = state.store.lock().unwrap();
        store.delete(&uuid);
    }
    Redirect::to("/")
}

// ─── JSON API Handlers ───

pub async fn api_list_users(State(state): State<AppState>) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    let users = store.list_all();
    (StatusCode::OK, axum::Json(users))
}

pub async fn api_get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    let result = get_user_by_id(&*store, &id);

    match result {
        GetUserByIdResult::Ok(user) => {
            (StatusCode::OK, axum::Json(serde_json::to_value(user).unwrap())).into_response()
        }
        GetUserByIdResult::NotFound { id } => {
            (
                StatusCode::NOT_FOUND,
                axum::Json(serde_json::json!({ "error": "not-found", "id": id })),
            )
                .into_response()
        }
        GetUserByIdResult::InvalidId { id } => {
            (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({ "error": "invalid-id", "id": id })),
            )
                .into_response()
        }
    }
}

pub async fn api_create_user(
    State(state): State<AppState>,
    axum::Json(input): axum::Json<CreateUserInput>,
) -> impl IntoResponse {
    let mut store = state.store.lock().unwrap();
    let result = create_user(&mut *store, input);

    match result {
        CreateUserResult::Ok(user) => {
            (StatusCode::CREATED, axum::Json(serde_json::to_value(user).unwrap())).into_response()
        }
        CreateUserResult::ValidationFailed(errors) => {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(serde_json::json!({ "error": "validation-failed", "errors": errors })),
            )
                .into_response()
        }
        CreateUserResult::DuplicateEmail { email } => {
            (
                StatusCode::CONFLICT,
                axum::Json(serde_json::json!({ "error": "duplicate-email", "email": email })),
            )
                .into_response()
        }
    }
}
