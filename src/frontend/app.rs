use chrono::{Local, Months, Days, Utc};
use async_std::task::block_on;
use reqwest::{Method, StatusCode};
use sycamore::prelude::*;
use crate::{string, frontend::{structs::{Auth, Tabs}, lib::{request, log, rfc_7231, HOST}}, entities::{LoginForm, LoginResult, RefreshResult}, utils::error::AppError};
use wasm_bindgen::JsCast;


#[component]
pub fn App() -> View {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let html_document2 = html_document.clone();
    let auth = create_signal(Auth::NotLogged);
    let tab = create_signal(Tabs::Main);
    let old_cookie = create_signal(string!(""));
    let cookie = html_document.cookie().unwrap();
    let device_signal: Signal<Option<String>>  = create_signal(match cookie.split("device=").nth(1){
        Some(part) => part.split(";").nth(0).map(|s|s.to_string()),
        None => None,
    });

    match cookie.split("device=").nth(1) {
        Some(part1) => {
            let device = part1.split(";").nth(0).unwrap();
            block_on(async move {
                match cookie.split("refresh=").nth(1) {
                    Some(first_part) => {
                        let token = first_part.split(";").nth(0).unwrap();
                        if token.len() > 10 {
                            let client = reqwest::Client::builder().build().unwrap();
                            let res = client
                                .request(Method::POST, &format!("{}/refresh_token", HOST.as_str()))
                                .header("Authorization", format!("Bearer {}", token))
                                .send()
                                .await;
                            let res = match res {
                                Ok(r) => match r.status() {
                                    StatusCode::OK => r
                                        .json::<RefreshResult>()
                                        .await
                                        .map_err(|e| AppError::HttpErr(72, e.to_string())),
                                    _ => Err(AppError::HttpErr(73, r.json::<String>().await.unwrap())),
                                },
                                Err(e) => Err(AppError::HttpErr(75, e.to_string())),
                            };
                            if let Ok(refresh_result) = res {
                                auth.set(Auth::Logged(LoginResult {
                                    id: refresh_result.id,
                                    token: refresh_result.token,
                                    refresh: token.to_string(),
                                }));
                            }
                        }
                    }
                    None => (),
                }
            });
        }
        None => {
            block_on(async move {
                let res = request::<String>(string!("register"),auth,Method::GET,None::<bool>,true).await.unwrap().unwrap();
                html_document.set_cookie(&format!("device={}; expires={}; path=/",&res,Local::now().checked_add_months(Months::new(60)).unwrap().to_utc().to_string())).unwrap();

            });
        },
    }
    create_memo(move || match auth.get_clone() {
        Auth::NotLogged => {
            html_document2.set_cookie(&format!("token={}", "")).unwrap();
            html_document2
                .set_cookie(&format!("refresh={}", ""))
                .unwrap();
            tab.set(Tabs::Main);
        }
        Auth::Logged(login) => {
            html_document2
                .set_cookie(&format!(
                    "token={}; expires={}; path=/",
                    &login.token,
                    rfc_7231(Utc::now().checked_add_days(Days::new(1)).unwrap())
                ))
                .unwrap();
            html_document2
                .set_cookie(&format!(
                    "refresh={}; expires={}; path=/",
                    &login.refresh,
                    rfc_7231(Utc::now().checked_add_days(Days::new(1)).unwrap())
                ))
                .unwrap();
        }
    });
    view!{
        div(class="app") {
            (match auth.get_clone() {
                Auth::Logged(tokens) => view!{
                    "Loaded!"
                    section(id="login"){
                        form(id="new_device_form"){
                            input(bind:value=old_cookie){}
                            input(r#type="submit",class="button"){"Login"}
                        }
                        button(class="button"){
                            "Generate new token"
                        }
                    }
                },
                Auth::NotLogged => view! {
                    (match device_signal.get_clone(){
                        Some(cookie) => {
                            block_on(async move {
                                let res = request::<LoginResult>(string!("login"),auth,Method::POST,Some(LoginForm{device:cookie}),true).await.unwrap().unwrap();
                                auth.set(Auth::Logged(res));
                            });
                            view!{
                                "Loading...!"
                            }
                        },
                        None => view!{
                            section(id="login"){
                                form(id="new_device_form"){
                                    input(bind:value=old_cookie){}
                                    input(r#type="submit",class="button"){"Login"}
                                }
                                button(class="button"){
                                    "Generate new token"
                                }
                            }
                        }
                    })
                }
            })
        }
    }
}