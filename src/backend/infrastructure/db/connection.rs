use std::env;
use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    Surreal,
};

pub type DBPool = Surreal<Any>;

pub struct PersonDB {
    token: String,
}

pub async fn establish_connection() -> DBPool {
    let db = connect(env::var("DB_URL").expect("DB URL not set"))
        .await
        .expect("Failed to establish connection");
    db.use_ns(env::var("ENV").expect("ENV not set"))
        .use_db("iglesia")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .unwrap();

    // Authenticate
    db.signin(Root {
        username: &env::var("DB_LOGIN").expect("DB LOGIN not set"),
        password: &env::var("DB_PASSWORD").expect("DB PASSWORD not set"),
    })
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    .unwrap();
    db
}

