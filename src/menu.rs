use sycamore::prelude::*;
use crate::app::Selector;

#[component(inline_props)]
pub fn Menu(menu: Signal<Selector>) -> View {

    view!{
        div(id = "menu"){
            a(class = format!("menu_a {}",
                match menu.get(){
                    Selector::Today => "selected",
                    _ => "",
                })){"Today"}
            a(class = format!("menu_a {}",
                match menu.get(){
                    Selector::Registry => "selected",
                    _ => "",
                })){"Registry"}
            a(class = format!("menu_a {}",
                match menu.get(){
                    Selector::Config => "selected",
                    _ => "",
                })){"Config"}
        }
    }
}