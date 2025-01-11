use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Сервис для обработки статических файлов
    let static_files = ServeDir::new("./static");

    // Определяем маршруты
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(hello_name))
        .route("/add", post(add_numbers))
        .route("/page", get(html_page)) // Новый маршрут для страницы
        .nest_service("/static", static_files); // Обслуживание статических файлов

    // Запускаем сервер
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущен на http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Корневой маршрут
async fn root() -> &'static str {
    "Простой REST-сервер на Rust с Axum и HTML/CSS"
}

// Обработчик для маршрута /hello/:name
async fn hello_name(Path(name): Path<String>) -> String {
    format!("Привет, {}!", name)
}

// Структуры для обработки JSON-запросов
#[derive(Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
struct AddResponse {
    sum: i32,
}

// Обработчик для маршрута /add (POST запрос)
async fn add_numbers(axum::Json(payload): axum::Json<AddRequest>) -> axum::Json<AddResponse> {
    let sum = payload.a + payload.b;
    axum::Json(AddResponse { sum })
}

// Новый обработчик для HTML-страницы
async fn html_page() -> impl IntoResponse {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>HTML Page</title>
            <link rel="stylesheet" href="/static/style.css">
        </head>
        <body>
            <h1>Пример страницы с CSS</h1>
            <p>Это пример страницы, которая использует стили из CSS-файла.</p>
        </body>
        </html>
        "#,
    )
}
