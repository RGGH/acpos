use actix_web::{
    error, get, post,
    web::{self, Json},
    App, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
use sqlx::{Executor, FromRow, Pool};

const DATABASE_URL: &str = "postgres://new_user:new_password@localhost/new_db";

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

#[get("/{id}")]
async fn retrieve(path: web::Path<i32>, state: web::Data<AppState>) -> Result<Json<Todo>> {
    // Query database to get data
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[post("")]
async fn add(todo: web::Json<TodoNew>, state: web::Data<AppState>) -> Result<Json<Todo>> {
    // Query database to create a new record using the request body
    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind(&todo.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(todo))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging

    // Load the DATABASE_URL environment variable
    let database_url = DATABASE_URL;

    use sqlx::postgres::PgPoolOptions;

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");


    // Run migrations
    pool.execute(include_str!("../schema.sql"))
        .await
        .expect("Failed to run migrations");

    // Set up AppState
    let state = web::Data::new(AppState { pool });

    // Set up and run the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/todos").service(retrieve).service(add))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
