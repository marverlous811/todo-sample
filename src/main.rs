use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::mysql::repository::TodoRepository;

pub mod mysql;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, env, default_value = "127.0.0.1:8080")]
    http_addr: SocketAddr,

    #[arg(long, env, default_value = "mysql://root:123@localhost:3306/cloud")]
    sql_dsn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodo {
    name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorMessage {
    message: String,
}

#[derive(Clone)]
pub struct App {
    todo: TodoRepository,
}

async fn create_todo(State(state): State<App>, Json(payload): Json<CreateTodo>) -> Response {
    match state.todo.create(&payload.name).await {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage { message: e.to_string() }),
        )
            .into_response(),
    }
}

async fn get_todo(State(state): State<App>, Path(id): Path<u32>) -> Response {
    match state.todo.find_id(id).await {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorMessage {
                message: "not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorMessage { message: e.to_string() }),
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Args::parse();

    let sql = mysql::SqlAdapter::new(&args.sql_dsn).await;
    sql.migrate(None).await?;

    let repository = sql.repository();
    let app = App { todo: repository };

    let app = Router::new()
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(get_todo))
        .with_state(app);

    let addr = args.http_addr;
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to socket");
    log::info!("[serve_http_transport] listening on {}", addr);
    axum::serve(listener, app).await.expect("Failed to start server");

    Ok(())
}
