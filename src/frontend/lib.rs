use crate::{entities::{LoginResult, RefreshResult},string};
use crate::utils::error::{AppError, AppRes};
use crate::frontend::structs::Auth;
use chrono::{DateTime, Datelike, Utc};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::LazyLock;
use sycamore::prelude::Signal;
use sycamore::prelude::*;
use web_sys::HtmlDocument;
use wasm_bindgen::JsCast;


const NAME: &'static str = "Lib";

pub static HOST: LazyLock<String> = LazyLock::new(|| std::env!("BACKEND").to_string());


pub fn document() -> HtmlDocument {
    web_sys::window().unwrap().document().unwrap().dyn_into::<web_sys::HtmlDocument>().unwrap()
}

//const HOST: &str = "http://localhost:8088/";

// pub async fn refresh_users(miembros: Signal<Vec<Persona>>, auth: Signal<Auth>) {
//     miembros.set(
//         request::<Vec<Persona>>("api/v1/users/", auth, Method::GET, None::<bool>, true)
//             .await
//             .unwrap()
//             .unwrap_or_default(),
//     );
// }
async fn fetch<T: DeserializeOwned>(
    url: &str,
    token: String,
    method: Method,
    body: Option<impl Serialize + ?Sized + Clone>,
    expects: bool,
) -> AppRes<Option<T>> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .request(method.clone(), url)
        .header("Authorization", format!("Bearer {}", token));
    let res = match body {
        None => req.send().await,
        Some(body) => req.json(&body).send().await,
    };
    // log(NAME, 41, &res);
    match res {
        Ok(r) => match r.status() {
            StatusCode::OK => {
                if expects {
                    r.json::<T>()
                        .await
                        .map_err(|e| AppError::HttpErr(46, e.to_string()))
                        .map(|t| Some(t))
                } else {
                    Ok(None)
                }
            }
            StatusCode::NO_CONTENT => Ok(None),
            other_status => Err(AppError::HttpErr(
                49,
                format!(
                    "Status: {}, \nMessage: {}",
                    other_status,
                    match r.json::<String>().await {
                        Ok(v) => v,
                        Err(e) => e.to_string(),
                    }
                ),
            )),
        },
        Err(e) => Err(AppError::HttpErr(54, e.to_string())),
    }
}

pub fn rfc_7231(date: DateTime<Utc>) -> String {
    format!(
        "{}, {} {} GMT",
        date.weekday().to_string().split_at(3).0,
        date.format("%d-%b-%Y"),
        date.format("%H:%M:%S")
    )
}
pub async fn request<T: DeserializeOwned>(
    url: impl AsRef<str>,
    login: Signal<Auth>,
    method: Method,
    body: Option<impl Serialize + ?Sized + Clone>,
    expects: bool,
) -> AppRes<Option<T>> {
    match login.get_clone_untracked() {
        Auth::NotLogged => {
            fetch::<T>(
                &format!("{}/{}", HOST.as_str(), url.as_ref()),
                string!("not-logged"),
                method.clone(),
                body.clone(),
                expects,
            )
                .await

        },
        Auth::Logged(_) => {
            let auth = login.get_clone_untracked().unwrap().clone();
            match fetch::<T>(
                &format!("{}/{}", HOST.as_str(), url.as_ref()),
                auth.token.clone(),
                method.clone(),
                body.clone(),
                expects,
            )
            .await
            {
                Ok(res) => Ok(res),
                Err(e) => {
                    log(NAME,102,&e);
                    match fetch::<RefreshResult>(
                        &format!("{}/refresh_token", HOST.as_str()),
                        auth.refresh.clone(),
                        Method::POST,
                        None::<bool>,
                        expects,
                    )
                    .await
                    {
                        Ok(refresh) => {
                            log(NAME, 77, &refresh);
                            login.set_fn(|result| {
                                let result = result.unwrap();
                                Auth::Logged(LoginResult {
                                    token: refresh.as_ref().unwrap().token.clone(),
                                    ..result.clone()
                                })
                            });
                            fetch::<T>(
                                format!("{}/{}", HOST.as_str(), url.as_ref()).as_str(),
                                refresh.unwrap().token,
                                method,
                                body,
                                expects,
                            )
                            .await
                            .map_err(|e| e.into())
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }
}

pub fn log<T: Debug>(file: &str, pos: u16, data: &T) {
    console_log!("{} {}:\n{:?}", file, pos, data);
}
