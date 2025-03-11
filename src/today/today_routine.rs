use sycamore::prelude::*;
use sycamore::rt::{console_error, spawn_local_scoped};
use structs::day::{Day, DayState};
use structs::routine::Routine;
use web_sys::MouseEvent;
use crate::libs::call;

#[component(inline_props)]
pub fn TodayRoutine(routine: Signal<Option<Routine>>) -> View {
    let today = create_signal(routine.get_clone().unwrap().today().cloned().unwrap());
    let td = today.clone();
    let td2 = today.clone();

    let state = create_selector( move || td.get_clone().state());
    let rt = routine.clone();
    let rt2 = routine.clone();
    create_memo(move || td2.set(rt2.get_clone().unwrap().today().cloned().unwrap()));

    view!{
        (match state.get(){
            DayState::Free => {
                view!{
                    button(on:click = move |ev:MouseEvent|{
                        spawn_local_scoped(async move {
                            match call::<Day>("check_in", None::<bool>).await {
                                Ok(day) => rt.update(|rt|{rt.as_mut().unwrap().this_week_mut().unwrap().set_today(day);}),
                                Err(e) => console_error!("Error: {}",e.to_string()),
                            }
                        })
                    }){"Check In"}
                }
            },
            DayState::Checked => {
                let res = today.get_clone().exercises().clone().into_iter().map(|ex|{
                    let (signals, views): (Vec<Signal<String>>,Vec<View>) = ex.series().into_iter().cloned().filter_map(|series|{
                        series.map(|sr|{let signal = create_signal(String::new());(signal,view!{input(bind:value=signal,placeholder = sr.count().to_string()){}})})
                    }).unzip();
                    let name = ex.name().to_string();
                    view!{
                        section(class = "series_container"){
                            p(){(name)}
                            (views)
                        }
                    }
                }).collect::<Vec<View>>();
                view!{
                    p(){"Checked Page"}
                (res)
            }},
            DayState::Complete => view!{},
        })

    }
}