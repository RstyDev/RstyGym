use std::env;
use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    Surreal,
};

pub type DBPool = Surreal<Any>;



pub async fn establish_connection() -> DBPool {
    let db = connect(env::var(String::from("DB_URL")).expect("DB URL not set"))
        .await
        .expect("Failed to establish connection");
    db.use_ns(env::var(String::from("ENV")).expect("ENV not set"))
        .use_db("gym")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .unwrap();

    // Authenticate
    db.signin(Root {
        username: &env::var(String::from("DB_LOGIN")).expect("DB LOGIN not set"),
        password: &env::var(String::from("DB_PASSWORD")).expect("DB PASSWORD not set"),
    })
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    .unwrap();
    db
}

