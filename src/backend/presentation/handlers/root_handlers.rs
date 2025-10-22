use crate::{
    backend::infrastructure::db::{DBPool, Record},
    entities::{Claims, LoginForm, LoginResult, RefreshResult, TokenType},
    error::AppError,
};
use actix_web::{dev::ServiceRequest, error::{self, Error as ActixError}, get, post, web::{Data, Json}, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Uuid};




#[get("/register")]
pub async fn register(db: Data<DBPool>) -> impl Responder {
    let res = db.query(r#"CREATE user"#).await;
    match res {
        Ok(mut res) => {
            let record: Option<Record> = res.take(0usize).unwrap();
            println!("Res: {:?}", record);
            HttpResponse::Ok().json(record.unwrap().id.key().to_string())
        },
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
#[post("/login")]
pub async fn login(db: Data<DBPool>, user: Json<LoginForm>) -> impl Responder {
    let LoginForm {
        device
    } = user.into_inner();
    println!("{:#?}", device);
    if device.len() > 0 {
        let user: Option<Record> = db.select(("user",device)).await.unwrap();
        match user {
            Some(rec) => {
                let id = rec.id.key().to_string();
                let token = get_token(Duration::minutes(5), TokenType::Normal, id.clone());
                let refresh = get_token(Duration::days(1), TokenType::Refresh, id.clone());
                let res = LoginResult { id, token, refresh };
                HttpResponse::Ok().json(res)
            },
            None => loc_register(db).await,
        }
    } else {
        loc_register(db).await
    }
}

async fn loc_register(db: Data<DBPool>) -> HttpResponse {
    let res = db.query(r#"CREATE user"#).await;
    match res {
        Ok(mut res) => {
            let record: Option<Record> = res.take(0usize).unwrap();
            println!("Res: {:?}", record);
            HttpResponse::Ok().json(record.unwrap().id.key().to_string())
        },
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
fn get_token(duration: Duration, tipo: TokenType, id: String) -> String {
    let now = Utc::now();
    let secret = match tipo {
        TokenType::Refresh => env::var(String::from("REFRESH_SECRET")).unwrap(),
        TokenType::Normal => env::var(String::from("SECRET")).unwrap(),
    };
    let claims = Claims {
        nbf: now.timestamp() as usize,
        iat: now.timestamp() as usize,
        exp: (now + duration).timestamp() as usize,
        tipo,
        id,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .unwrap()
}

fn validate_token(token: String, tipo: TokenType) -> Result<Claims, AppError> {
    let secret = match tipo {
        TokenType::Refresh => env::var(String::from("REFRESH_SECRET")).unwrap(),
        TokenType::Normal => env::var(String::from("SECRET")).unwrap(),
    };
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let res = decode(&token, &decoding_key, &Validation::default());
    match res {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(AppError::ValidationErr(44, err.to_string())),
    }
}

#[post("/refresh_token")]
pub async fn refresh_token(refresh_jwt: Option<BearerAuth>) -> HttpResponse {
    let Some(refresh) = refresh_jwt else {
        return HttpResponse::Forbidden().json("Token no enviado");
    };
    match validate_token(refresh.token().to_string(), TokenType::Refresh) {
        Ok(c) => {
            let token = get_token(Duration::minutes(5), TokenType::Normal, c.id.clone());
            let res = RefreshResult { id: c.id, token };
            HttpResponse::Ok().json(res)
        }
        Err(_) => HttpResponse::Unauthorized().json("Token invalid"),
    }
}

// #[post("/login_by_token")]
// pub async fn login_by_token()

pub async fn validator(
    req: ServiceRequest,
    credenciales: Option<BearerAuth>,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let Some(cred) = credenciales else {
        return Err((error::ErrorBadRequest(String::from("No se recibió el token")), req));
    };
    let token = cred.token().to_string();
    match validate_token(token, TokenType::Normal) {
        Ok(_) => Ok(req),
        Err(_) => Err((error::ErrorForbidden(String::from("No tiene acceso")), req)),
    }
}
