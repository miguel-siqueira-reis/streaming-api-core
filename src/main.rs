
use std::env;
use AnimeNull::state::{AppConfig, AppState};
use AnimeNull::{telemetry, worker, features};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Carrega variáveis de ambiente do arquivo .env
    dotenvy::dotenv().ok();

    // 0. Inicializa o Sistema de Logs
    telemetry::init_subscriber();

    tracing::info!("Inicializando Servidor AnimeNull...");

    // 1. Variáveis Ambientes
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/animenull".to_string());
    let config = AppConfig::from_env();

    // 2. Prepara Dependências
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_url).await?;

    // Roda as Migrações
    sqlx::migrate!("./migrations").run(&pool).await?;

    let (worker_tx, worker_rx) = tokio::sync::mpsc::channel(100);

    // 3. Monta o Estado Principal
    let state = AppState::new(pool, worker_tx, config);

    // 4. Inicia Workers num fio isolado
    tokio::spawn(worker::run(worker_rx, state.clone()));

    // 5. Constrói Roteamento Central em Fatias + Camadas de Segurança e Trace
    use tower_http::cors::CorsLayer;
    use tower_http::trace::TraceLayer;

    let app = features::router(state)
        .layer(TraceLayer::new_for_http()) // Esse carinha vai logar TODO request e response automaticamente!
        .layer(CorsLayer::permissive());

    // 6. Configura a Porta e Inicia
    let port = "0.0.0.0:4000";
    let listener = tokio::net::TcpListener::bind(port).await?;
    tracing::info!("🚀 Motor ligou: Servidor rodando em http://{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
