use chrono::{Datelike, Local, Weekday};
use crate::today::add_routine::AddRoutine;
use structs::routine::Routine;
use sycamore::prelude::*;
use crate::libs::log;
use crate::today::today_routine::TodayRoutine;

#[component(inline_props)]
pub fn Today(routine: Signal<Option<Routine>>) -> View {
    let r1 = routine.clone();
    let r2 = routine.clone();
    let routine_selector = create_selector(move || r1.get_clone());
    create_memo( move ||{log("Desde memoooo",12,&r2.get_clone())});
    log("From Today",11,&routine_selector.get_clone().map(|r|{r.today().cloned()}));
    view! {
        (match routine_selector.get_clone(){
            Some(rtn) if rtn.today().is_some() => view!{ // El || se va
                TodayRoutine(routine = routine.clone())
            },
            Some(_) if Local::now().date_naive().weekday() == Weekday::Sun => {
                view!{
                    p(){"Sunday!"}
                }
            }
            _ => view!{
                AddRoutine(routine = routine.clone())
            }
        })
    }
}
