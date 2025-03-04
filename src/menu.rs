use crate::app::Selector;
use sycamore::prelude::*;
use web_sys::MouseEvent;

#[component(inline_props)]
pub fn Menu(menu: Signal<Selector>) -> View {
    let (m1, m2, m3) = (menu.clone(), menu.clone(), menu.clone());

    view! {
        div(id = "menu"){
            a(class = format!("menu_a {}",
                match menu.get(){
                    Selector::Today => "selected",
                    _ => "",
                }),on:click= move |_|{m1.set(Selector::Today)}){"Today"}
            a(class = format!("menu_a {}",
                match menu.get(){
                    Selector::Registry => "selected",
                    _ => "",
                }),on:click= move |_|{m2.set(Selector::Registry)}){"Registry"}
            a(class = format!("menu_a {}",
                match menu.get(){
                    Selector::Config => "selected",
                    _ => "",
                }),on:click= move |_|{m3.set(Selector::Config)}){"Config"}
        }
    }
}
