use sycamore::prelude::*;
use sycamore::rt::{console_error, spawn_local_scoped};
use structs::day::DayState;
use structs::routine::Routine;
use web_sys::MouseEvent;
use crate::libs::call;

#[component(inline_props)]
pub fn TodayRoutine(routine: Signal<Option<Routine>>) -> View {
    let today = create_signal(routine.get_clone().unwrap().today().cloned().unwrap());
    let td = today.clone();
    let state = create_selector( move || td.get_clone().state());
    let rt = routine.clone();

    view!{
        (match state.get(){
            DayState::Free => {
                view!{
                    button(on:click = move |ev:MouseEvent|{
                        spawn_local_scoped(async move {
                            if let Err(e) = call::<()>("check_in", None::<bool>).await{
                                console_error!("Error: {}",e.to_string())
                            } else {
                                rt.update(|rt|{rt.as_mut().unwrap().this_week_mut().unwrap().today_mut().unwrap().set_state(DayState::Checked);});
                            }
                        })
                    }){"Check In"}
                }
            },
            DayState::Checked => {
                let res = today.get_clone().exercises().into_iter().map(|ex|{
                    let (signals, views): (Vec<Signal<String>>,Vec<View>) = ex.series().into_iter().cloned().filter_map(|series|{
                        series.map(|sr|{let signal = create_signal(String::new());(signal,view!{input(bind:value=signal,placeholder = sr.count().to_string()){}})})
                    }).unzip();
                    view!{
                        (views)
                    }
                }).collect::<Vec<View>>();
                view!{
                (res)
            }},
            DayState::Complete => view!{},
        })

    }
}