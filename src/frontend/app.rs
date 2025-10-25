use chrono::{Days, Utc};
use async_std::task::block_on;
use reqwest::{Method, StatusCode};
use sycamore::prelude::*;
use crate::{string, frontend::{structs::{Auth, Tabs},pages::Login, lib::{rfc_7231, HOST, document}}, entities::{LoginResult, RefreshResult}, utils::error::AppError};


#[component]
pub fn App() -> View {
    let html_document = document();
    let html_document2 = html_document.clone();
    let auth = create_signal(Auth::NotLogged);
    let tab = create_signal(Tabs::Main);
    let old_cookie = create_signal(string!(""));
    let device_signal: Signal<Option<String>>  = create_signal(match html_document.cookie().unwrap().split("device=").nth(1){
        Some(part) => {
            let part = part.split(";").nth(0).map(|s| s.to_string()).unwrap();
            (part.len()>0).then_some(part)
        },
        None => None,
    });
    let html = html_document.clone();
    let html2 = html.clone();

    match html_document.cookie().unwrap().split("device=").nth(1) {
        Some(part1) => {
            if let Some(_) = part1.split(";").nth(0) {
                block_on(async move {
                    match html2.cookie().unwrap().split("refresh=").nth(1) {
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
        }
        None => {

        },
    }
    create_memo(move || if device_signal.with(|d|d.is_none()){
        auth.set(Auth::NotLogged)
    });
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
                Auth::Logged(_) => {
                    let html = html.to_owned();
                    view!{
                        (match tab.get(){
                            Tabs::Main => view!{

                            },
                            Tabs::History => view! {

                            },
                            Tabs::Configs => {
                                let html = html.to_owned();
                                view! {
                                    button(class="button",on:click=move |_| {
                                        html.set_cookie(format!("device={}; expires={}; path=/","","").as_str()).unwrap();
                                        device_signal.set_silent(None);
                                        auth.set(Auth::NotLogged);
                                    }){
                                        "Log Out"
                                    }
                                }
                            },
                        })
                        section(id="tab_nav"){
                            button(on:click=move|_|{
                                tab.set(Tabs::Main)
                            }){"Main"}
                            button(on:click=move|_|{
                                tab.set(Tabs::History)
                            }){"History"}
                            button(on:click=move|_|{
                                tab.set(Tabs::Configs)
                            }){"Configs"}
                        }
                    }

                },
                Auth::NotLogged => view! {
                    Login(auth = auth, device_signal=device_signal, old_cookie = old_cookie)
                }
            })
        }
    }
}