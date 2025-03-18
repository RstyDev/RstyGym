use crate::commands::NewRoutine;
use crate::libs::{call, log};
use crate::today::add_exercise::AddExercise;
use chrono::{Days, Local};
use serde_wasm_bindgen::from_value;
use structs::day_template::DayTemplate;
use structs::exercise::Exercise;
use structs::routine::Routine;
use structs::week::Week;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::{console_error, spawn_local};
use web_sys::MouseEvent;

#[derive(Clone, Copy, PartialEq, Default)]
enum State {
    #[default]
    NotAdding,
    AddingWeekTemplate,
    AddingDayTemplate,
    AddingExercise,
}
fn new_routine(name: String, mut rtn: Routine, routine: Signal<Option<Routine>>) {
    spawn_local_scoped(async move {
        match call::<i64>(
            "new_routine",
            Some(NewRoutine {
                name,
                routine: rtn.clone(),
            }),
        )
        .await
        {
            Ok(id) => {
                rtn.set_id(id);
                routine.set(Some(rtn));
            }
            Err(e) => console_error!("{e}"),
        }
    });
}
#[component(inline_props)]
pub fn AddRoutine(routine: Signal<Option<Routine>>) -> View {
    fn default<T: Default>(data: Signal<T>) {
        data.set(T::default());
    }
    let rtn_signal = routine.clone();
    let name = create_signal(String::new());
    let week_template: Signal<Vec<DayTemplate>> = create_signal(Vec::new());
    let day_template = create_signal(DayTemplate::default());
    let exercise = create_signal(Exercise::default());
    let ex1 = exercise.clone();
    let state = create_signal(State::NotAdding);
    let st1 = state.clone();
    let (w, d, e, s) = (
        week_template.clone(),
        day_template.clone(),
        exercise.clone(),
        state.clone(),
    );
    let cancel = move |_: MouseEvent| {
        default(w);
        default(d);
        default(e);
        default(s);
    };

    let (tmp1, tmp2, tmp3, tmp4) = (
        week_template.clone(),
        week_template.clone(),
        week_template.clone(),
        week_template.clone(),
    );
    let tmp = tmp1.clone();
    let tmp_selector = create_selector(move || tmp.with(|a| a.len() > 0));
    let (dy_tmp, dy2, dy3, dy4, dy5, dy6) = (
        day_template.clone(),
        day_template.clone(),
        day_template.clone(),
        day_template.clone(),
        day_template.clone(),
        day_template.clone(),
    );
    let dy = dy5.clone();
    let dy_selector = create_selector(move || dy.with(|d| d.exercises().len() > 0));
    let dy_selector2 = dy_selector.clone();
    create_memo(move || log("WeekTemplate", 79, &tmp4.get_clone()));
    create_memo(move || log("DayTemplate", 80, &dy4.get_clone()));
    let rtn1 = routine.clone();
    let (s1, s2, s3, s4, s5, s6, s7, s8) = (
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
    );

    let state_selector = create_selector(move || state.get());
    create_memo(move || log("Exercise", 94, &ex1.get_clone()));
    view! {
        (match state_selector.get(){
            State::NotAdding => view!{},
            _ => view!{input(placeholder = "Created By", bind:value=name){}}
        })
        (match state_selector.get(){
            State::NotAdding => view!{
                button(on:click =move |_|{
                    s1.set(State::AddingWeekTemplate);
                }){"Add Template"}
            },
            State::AddingWeekTemplate => {
                view!{
                    section(class = "frame"){
                        (match tmp_selector.get(){
                            true => {
                                let dato = tmp3.get_clone();
                                let days_view = tmp3.get_clone().into_iter().enumerate().map(|(i,day)|view!{
                                    article(){
                                        (format!("Day {}, {} exercises",i+1,day.exercises().len()))
                                    }
                                }).collect::<Vec<View>>();
                                view!{(days_view)}
                            }
                            false => {view!{article(){"-"}}}
                        })

                    }
                    button(on:click =move |_|{
                        s6.set(State::AddingDayTemplate)
                    }){"Add Day"}
                    button(on:click= move |_|{
                        let today = Local::now().date_naive();
                        let rtn = Routine::build(
                            None,
                            week_template.get_clone(),
                            [
                                Week::build_from_day(today),
                                Week::build_from_day(today.checked_add_days(Days::new(7)).unwrap()),
                                Week::build_from_day(today.checked_add_days(Days::new(14)).unwrap()),
                                Week::build_from_day(today.checked_add_days(Days::new(21)).unwrap()),
                            ],
                            None,
                            None,
                            Some(String::from("Lucas")),
                            Local::now().date_naive());
                        new_routine(name.get_clone(),rtn, rtn_signal)
                    }){"Done"}
                    button(on:click = cancel ){"Cancel"}
                }
            },
            State::AddingDayTemplate => view!{
                section(class = "frame"){
                    (match dy_selector2.get(){
                        true => {
                            let data = dy6.with(|d|{d.exercises().clone()});
                            let view = data.into_iter().map(|ex|view!{
                                article(){
                                    (ex.name().to_string())
                                }
                            }).collect::<Vec<View>>();
                            view!{
                                (view)
                            }
                        }
                        false => {view!{article(){"-"}}}
                    })
                }
                button(on:click = move |_|{
                    s1.set(State::AddingExercise);
                }){"Add Exercise"}
                button(on:click = move |_| {
                    week_template.update(|w|{w.push(day_template.get_clone())});
                    default(dy5);
                    s1.set(State::AddingWeekTemplate)
                }){"Done"}
                button(on:click = move |_|{
                    s5.set(State::AddingWeekTemplate)
                }){"Back"}
                button(on:click = cancel ){"Cancel"}
            },
            State::AddingExercise => view!{
                section(class = "frame"){
                    (match dy_selector.get(){
                        true => {
                            let data = dy5.with(|d|{d.exercises().clone()});
                            let view = data.into_iter().map(|ex|view!{
                                article(){
                                    (ex.name().to_string())
                                }
                            }).collect::<Vec<View>>();
                            view!{
                                (view)
                            }
                        }
                        false => {
                            view!{article(){"-"}}
                        }
                    })
                }
                AddExercise(exercise = exercise)
                button(on:click = move |_|{
                    let ex = exercise.get_clone();
                    exercise.set_silent(Exercise::default());
                    st1.set(State::AddingDayTemplate);
                    day_template.update(move |day|{
                        day.exercises_mut().push(ex);
                    });
                }){"Add"}
                button(on:click = move |_|{
                    s3.set(State::AddingDayTemplate)
                }){"Back"}
                button(on:click = cancel ){"Cancel"}
            }
        })

    }
}
