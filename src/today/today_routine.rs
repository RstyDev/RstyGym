use crate::libs::{call, log};
use structs::day::{Day, DayState};
use structs::routine::Routine;
use sycamore::prelude::*;
use sycamore::rt::{console_error, spawn_local_scoped};
use web_sys::{KeyboardEvent, MouseEvent};
use crate::commands::UpdateWeight;

#[component(inline_props)]
pub fn TodayRoutine(routine: Signal<Option<Routine>>) -> View {
    let today = create_signal(routine.get_clone().unwrap().today().cloned().unwrap());
    let last_day = create_signal(routine.get_clone().unwrap().last_day(&today.get_clone()).cloned());
    let td = today.clone();
    let td2 = today.clone();

    let state = create_selector(move || td.get_clone().state());
    let rt = routine.clone();
    let rt2 = routine.clone();

    log("TodayRoutine ",19, &today.get_clone());
    create_memo(move || td2.set(rt2.get_clone().unwrap().today().cloned().unwrap()));

    view! {
        (match state.get(){
            DayState::Free => {
                view!{
                    button(on:click = move |ev:MouseEvent|{
                        spawn_local_scoped(async move {
                            match call::<(Day, Option<Day>)>("check_in", None::<bool>).await {
                                Ok((today, last_day)) => rt.update(|rt|{rt.as_mut().unwrap().this_week_mut().unwrap().set_today(today);}),
                                Err(e) => console_error!("Error: {}",e.to_string()),
                            }
                        })
                    }){"Check In"}
                }
            },
            DayState::Checked => {

                let res = today.get_clone().exercises().clone().into_iter().enumerate().map(|(ex_index,ex)|{
                    // let last_ex = last_day.get_clone().map(|d|{d.exercise_at(ex_index).unwrap().clone()});
                    let (signals, views): (Vec<Signal<String>>,Vec<View>) = ex.series().into_iter().enumerate().filter_map(|(i,series)|{
                        log("Day: ",42, &(last_day.get_clone(), ex_index, i));
                        let last_series = match last_day.get_clone() {
                            None=> None,
                            Some(d)=> match d.exercise_at(ex_index) {
                                Ok(ex) => {ex.series_at(i).cloned()}
                                Err(_) => None,
                            }
                        };
                        series.clone().map(move |sr|{
                            let signal = create_signal(
                            series.as_ref().map(|s|s.weight().unwrap_or(&0.0).to_string()).unwrap_or_default());
                            let input_value = last_series.map(|s|{s.weight().map(|w|w.to_string()).unwrap_or(String::from("-"))}).unwrap_or(String::from("-"));
                            let s1 = signal.clone();
                            create_effect(move ||{
                                let input = signal.get_clone();
                                if input.len()>0 {
                                    let weight = match input.parse::<f32>(){
                                        Ok(f) => f,
                                        Err(e) => {console_error!("Error: {e}");0.0}
                                    };
                                    // let weight = signal.with(|s|s.parse::<f32>().unwrap());
                                    spawn_local_scoped(async move {
                                        call::<()>("update_weight",Some(UpdateWeight{
                                            exerciseIndex: ex_index as u8,
                                            index: i as u8,
                                            weight
                                        })).await.unwrap();
                                    })
                                }
                            });
                            (signal.clone(),view!{
                                section(class = "input_pair"){
                                    input(
                                        disabled = true,
                                        value = input_value,
                                    )
                                    article(){
                                        input(
                                            bind:value=signal,
                                            r#type = "number",
                                            step = "0,01",
                                            placeholder = sr.weight().as_ref().map(|w|w.to_string()).unwrap_or_default()
                                        ){}
                                        input(disabled = true, value = "Kg"){}
                                    }
                                }
                            })
                        })
                    }).unzip();
                    let name = ex.name().to_string();
                    view!{
                        section(class = "series_container"){
                            p(){(name)}
                            section(class="input_pair"){
                                input(disabled = true, value = "Last"){}
                                input(disabled = true, value = "Current"){}
                            }
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
