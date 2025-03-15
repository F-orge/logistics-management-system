use lib_entity::generated::users;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, Database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect(std::env::var("RUST_DATABASE_URL")?).await?;

    let (email, password) = ("test@email.com", "RandomPassword1");

    // test users
    let mut user = users::ActiveModel::new();

    user.email = sea_orm::Set(email.into());
    user.password = sea_orm::Set(password.into());
    user.auth_type = sea_orm::Set("basic_auth".into());

    let user = user.insert(&db).await?;

    // file service permissions

    Ok(())
}
