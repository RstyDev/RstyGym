use sycamore::prelude::*;
use crate::{string, frontend::{structs::Auth, lib::{request, log,document}}, entities::{LoginResult, LoginForm}};
use reqwest::Method;
use async_std::task::block_on;
use chrono::{Months, Local};
use web_sys::SubmitEvent;
#[component(inline_props)]
pub fn Login(auth: Signal<Auth>,device_signal: Signal<Option<String>>, old_cookie: Signal<String>) -> View {
    
    view!{
        (match device_signal.get_clone(){
            Some(cookie) => {
                log("Login",10,&cookie);
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
                    form(id="new_device_form",on:submit=move |e:SubmitEvent|{
                        e.prevent_default();
                        block_on(async move {
                            match request::<LoginResult>(string!("login"),auth,Method::POST,Some(LoginForm{device:old_cookie.get_clone()}),true).await {
                                Ok(res) => {
                                    match res {
                                        Some(logger) => auth.set(Auth::Logged(logger)),
                                        None => log("Login",31,&"Result form empty")
                                    }
                                },
                                Err(e) => {
                                    log("Login",35,&e);
                                }
                            }
                            
                        });
                    }){
                        input(name="old_cookie",bind:value=old_cookie,placeholder="Your device"){}
                        input(r#type="submit",class="button"){"Login"}
                        button(class="button",on:click=move|_|{
                            block_on(async move {
                                let res = request::<String>(string!("register"),auth,Method::GET,None::<bool>,true).await.unwrap().unwrap();
                                document().set_cookie(&format!("device={}; expires={}; path=/",&res,Local::now().checked_add_months(Months::new(60)).unwrap().to_utc().to_string())).unwrap();
                                device_signal.set(Some(res));
                            });
                        }){
                            "Generate new token"
                        }
                    }
                }
            }
        })

    }
}