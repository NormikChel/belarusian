use askama::Template;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "history.html")]
struct HistoryTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "grammar.html")]
struct GrammarTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "vocabulary.html")]
struct VocabTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "taraskievica.html")]
struct TaraTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "dialects.html")]
struct DialTmpl { active: &'static str }

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTmpl { active: &'static str }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/history", get(history))
        .route("/grammar", get(grammar))
        .route("/vocabulary", get(vocabulary))
        .route("/taraskievica", get(taraskievica))
        .route("/dialects", get(dialects))
        .route("/about", get(about))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Моўны сэрвэр круціцца на http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> IndexTmpl { IndexTmpl { active: "index" } }
async fn history() -> HistoryTmpl { HistoryTmpl { active: "history" } }
async fn grammar() -> GrammarTmpl { GrammarTmpl { active: "grammar" } }
async fn vocabulary() -> VocabTmpl { VocabTmpl { active: "vocabulary" } }
async fn taraskievica() -> TaraTmpl { TaraTmpl { active: "taraskievica" } }
async fn dialects() -> DialTmpl { DialTmpl { active: "dialects" } }
async fn about() -> AboutTmpl { AboutTmpl { active: "about" } }