use crate::{
    string,
    backend::infrastructure::db::{DBPool, Record},
    entities::{Claims, LoginForm, LoginResult, RefreshResult, TokenType},
};
use actix_web::{dev::ServiceRequest, error::{self, Error as ActixError}, get, post, web::{Data, Json}, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Uuid};
use crate::utils::error::{AppError, AppRes};

#[get("/register")]
pub async fn register(db: Data<DBPool>) -> impl Responder {
    let res = db.query(r#"CREATE device"#).await;
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
pub async fn login(db: Data<DBPool>, device: Json<LoginForm>) -> impl Responder {
    let LoginForm {
        device
    } = device.into_inner();
    println!("{:#?}", device);
    if device.len() > 0 {
        let device: Option<Record> = db.select(("device",device)).await.unwrap();
        match device {
            Some(rec) => {
                let id = rec.id.key().to_string();
                let token = get_token(Duration::minutes(5), TokenType::Normal, id.clone());
                let refresh = get_token(Duration::days(1), TokenType::Refresh, id.clone());
                let res = LoginResult { id, token, refresh };
                HttpResponse::Ok().json(res)
            },
            None => match loc_register(db).await{
                Ok(id) => {
                    let token = get_token(Duration::minutes(5), TokenType::Normal, id.clone());
                    let refresh = get_token(Duration::days(1), TokenType::Refresh, id.clone());
                    let res = LoginResult { id, token, refresh };
                    HttpResponse::Ok().json(res)
                }
                Err(e) => HttpResponse::InternalServerError().json(e),
            },
        }
    } else {
        match loc_register(db).await{
            Ok(id) => {
                let token = get_token(Duration::minutes(5), TokenType::Normal, id.clone());
                let refresh = get_token(Duration::days(1), TokenType::Refresh, id.clone());
                let res = LoginResult { id, token, refresh };
                HttpResponse::Ok().json(res)
            }
            Err(e) => HttpResponse::InternalServerError().json(e),
        }
    }
}

async fn loc_register(db: Data<DBPool>) -> AppRes<String> {
    let res = db.query(r#"CREATE device"#).await;
    match res {
        Ok(mut res) => {
            let record: Option<Record> = res.take(0usize).unwrap();
            Ok(record.unwrap().id.key().to_string())
        },
        Err(e) => Err(AppError::HttpErr(57,string!("Login error"))),
    }
}
fn get_token(duration: Duration, tipo: TokenType, id: String) -> String {
    let now = Utc::now();
    
    let secret = match tipo {
        TokenType::Refresh => env::var(string!("REFRESH_SECRET")).unwrap(),
        TokenType::Normal => env::var(string!("SECRET")).unwrap(),
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
        TokenType::Refresh => env::var(string!("REFRESH_SECRET")).unwrap(),
        TokenType::Normal => env::var(string!("SECRET")).unwrap(),
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
        return Err((error::ErrorBadRequest(string!("No se recibió el token")), req));
    };
    let token = cred.token().to_string();
    match validate_token(token, TokenType::Normal) {
        Ok(_) => Ok(req),
        Err(_) => Err((error::ErrorForbidden(string!("No tiene acceso")), req)),
    }
}
