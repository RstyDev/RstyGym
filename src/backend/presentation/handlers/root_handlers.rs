use crate::backend::infrastructure::db::{DBPool, PersonDB};
use crate::{
    entities::{Claims, LoginForm, LoginResult, RefreshResult, TokenType},
    error::AppError,
};
use actix_web::{
    dev::ServiceRequest,
    error::{self, Error as ActixError},
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;

#[post("/login")]
pub async fn login(db: Data<DBPool>, user: Json<LoginForm>) -> impl Responder {
    let LoginForm {
        nombre,
        apellido,
        password,
    } = user.into_inner();
    println!("{:#?}\n{:#?}\n{:#?}", nombre, apellido, password);
    if nombre.len() > 2 && apellido.len() > 2 && password.len() >= 6 {
        let res: Result<Vec<PersonDB>,_> = db.query("select * from personas where nombre == $nombre and apellido == $apellido and password == crypto::sha512($pass);").bind(("nombre",nombre)).bind(("apellido",apellido)).bind(("pass",password)).await.unwrap().take(0);
        println!("-.-.- 26 {:#?}", res);
        match res {
            Ok(d) => {
                if d.len() > 0 {
                    let hermano = &d[0];
                    let id = hermano.id().as_ref().unwrap().id.to_string();
                    let token = get_token(Duration::minutes(5), TokenType::Normal, id.clone());
                    let refresh = get_token(Duration::days(1), TokenType::Refresh, id.clone());
                    let res = LoginResult { id, token, refresh };
                    HttpResponse::Ok().json(res)
                } else {
                    HttpResponse::Unauthorized().json("Error de autenticación")
                }
            }
            Err(err) => HttpResponse::InternalServerError().json(err),
        }
    } else {
        HttpResponse::Unauthorized().json("Nombre, Apellido y Password no pueden estar vacíos")
    }
}

fn get_token(duration: Duration, tipo: TokenType, id: String) -> String {
    let now = Utc::now();
    let secret = match tipo {
        TokenType::Refresh => env::var("REFRESH_SECRET".into()).unwrap(),
        TokenType::Normal => env::var("SECRET".into()).unwrap(),
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
        TokenType::Refresh => env::var("REFRESH_SECRET".into()).unwrap(),
        TokenType::Normal => env::var("SECRET".into()).unwrap(),
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
        return Err((error::ErrorBadRequest("No se recibió el token".into()), req));
    };
    let token = cred.token().to_string();
    match validate_token(token, TokenType::Normal) {
        Ok(_) => Ok(req),
        Err(_) => Err((error::ErrorForbidden(String::from("No tiene acceso")), req)),
    }
}
