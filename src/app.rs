use crate::today::Today;
use crate::{libs::call, menu::Menu};
use serde::{Deserialize, Serialize};
use structs::routine::Routine;
use sycamore::{futures::spawn_local_scoped, prelude::*, web::events::SubmitEvent};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub enum Selector {
    Today,
    Registry,
    Config,
}
async fn get_state() -> Option<Routine> {
    match call::<Option<Routine>>("get_state", None::<bool>).await {
        Ok(d) => d,
        Err(e) => {
            console_log!("{e}");
            None
        }
    }
}

#[component]
pub fn App() -> View {
    let menu = create_signal(Selector::Today);
    let m1 = menu.clone();
    let menu_selector = create_selector(move || m1.get());
    let routine = create_signal(None);

    let r = routine.clone();
    spawn_local_scoped(async move { r.set(get_state().await) });
    view! {
        main(class="container") {
            h1 {
                "Welcome to Tauri + Sycamore"
            }

            div(class="row") {
                a(href="https://tauri.app", target="_blank") {
                    img(src="public/tauri.svg", class="logo tauri", alt="Tauri logo")
                }
                a(href="https://sycamore.dev", target="_blank") {
                    img(src="public/sycamore.svg", class="logo sycamore", alt="Sycamore logo")
                }
            }

            (match menu_selector.get(){
                Selector::Today => view!{
                    Today(routine = routine.clone())
                },
                Selector::Registry => {view!{}}
                Selector::Config => {view!{}}
            })
            Menu(menu = menu)
        }
    }
}
