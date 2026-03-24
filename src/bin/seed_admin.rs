use AnimeNull::features::auth::{crypto, repository::{self, RegisterUserData}};
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando o seeder do banco de dados...");

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("A variável DATABASE_URL precisa estar configurada no .env");

    let pool = PgPool::connect(&database_url).await?;
    println!("✓ Conectado ao banco de dados!");

    let admin_email = "admin@animenull.com";

    let existing_user = repository::find_user_by_email(&pool, admin_email).await?;
    if existing_user.is_some() {
        println!("⚠️ O usuário admin já existe no banco de dados!");
        return Ok(());
    }

    let raw_password = "admin_password123";
    let password_hash = match crypto::hash_password(raw_password) {
        Ok(hash) => hash,
        Err(_) => panic!("Falha ao criar o hash da senha"),
    };

    let user_data = RegisterUserData {
        id: uuid::Uuid::new_v4(),
        name: "Administrador Master".to_string(),
        username: "admin".to_string(),
        email: admin_email.to_string(),
        password_hash,
        role: "ADMIN".to_string(),
    };

    repository::create_user(&pool, user_data).await?;

    println!("✅ Usuário admin criado com sucesso!");
    println!("---");
    println!("Email: {}", admin_email);
    println!("Senha: {}", raw_password);
    println!("Role: ADMIN");
    println!("---");

    Ok(())
}
