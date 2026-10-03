use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Настраиваем роуты
    let app = Router::new().route("/", get(index_handler));

    // Порт подхватываем из окружения (обязательно для облачных хостингов вроде Northflank)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
        
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Пагнали! Сервер крутится на http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html lang="be">
        <head>
            <meta charset="UTF-8">
            <title>Беларуская мова на Rust</title>
            <style>
                body { font-family: sans-serif; background: #121212; color: #f0f0f0; text-align: center; padding-top: 50px; }
                h1 { color: #ffcc00; }
            </style>
        </head>
        <body>
            <h1>Мова жыве! 🚀</h1>
            <p>Гэты сайт напісаны на Rust і ганарліва лунае на халяўным хостынгу.</p>
        </body>
        </html>
    "#)
}