use sycamore::prelude::*;
use crate::{string, frontend::{structs::Auth, lib::{request, log,document}}, entities::{LoginResult, LoginForm}};
use reqwest::Method;
use async_std::task::block_on;
use chrono::{Months, Local};
use wasm_bindgen::JsCast;
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
                    form(id="new_device_form"){
                        input(bind:value=old_cookie){}
                        input(r#type="submit",class="button"){"Login"}
                    }
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
        })

    }
}