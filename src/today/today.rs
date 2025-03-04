use crate::today::add_routine::AddRoutine;
use structs::routine::Routine;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn Today(routine: Signal<Option<Routine>>) -> View {
    let r1 = routine.clone();
    let routine_selector = create_selector(move || r1.get_clone());
    console_log!("{:#?}", routine_selector.get_clone());
    view! {
        (match routine_selector.get_clone(){
            None => view!{
                AddRoutine(routine = routine.clone())
            },
            Some(routine) => view!{

            },
        })
    }
}
