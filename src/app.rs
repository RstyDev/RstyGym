use crate::libs::log;
use crate::today::Today;
use crate::{libs::call, menu::Menu};
use serde::{Deserialize, Serialize};
use structs::routine::Routine;
use sycamore::{futures::spawn_local_scoped, prelude::*};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug)]
pub enum Selector {
    Today,
    Registry,
    Config,
}
async fn get_state() -> Option<Routine> {
    match call::<Option<Routine>>("get_state", None::<bool>).await {
        Ok(d) => {
            log("Ok Routine", 18, &d);
            d
        }
        Err(e) => {
            log("Err Routine", 22, &e);
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
    let r2 = routine.clone();
    spawn_local_scoped(async move {
        let res = get_state().await;
        log("Updating Routine", 38, &res);
        r.set(res)
    });
    view! {
        main(class="container") {
            // h1 {
            //     "Welcome to Tauri + Sycamore"
            // }

            div(class="row") {
                a(href="https://tauri.app", target="_blank") {
                    img(src="public/tauri.svg", class="logo tauri", alt="Tauri logo")
                }
                a(href="https://sycamore.dev", target="_blank") {
                    img(src="public/sycamore.svg", class="logo sycamore", alt="Sycamore logo")
                }
            }
            section(id="body"){
                (match menu_selector.get(){
                    Selector::Today => view!{
                        Today(routine = r2.clone())
                    },
                    Selector::Registry => {view!{}}
                    Selector::Config => {view!{}}
                })
            }
            Menu(menu = menu)
        }
    }
}
