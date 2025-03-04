use crate::today::add_exercise::AddExercise;
use chrono::Local;
use structs::day_template::DayTemplate;
use structs::exercise::Exercise;
use structs::routine::Routine;
use structs::week::Week;
use sycamore::prelude::*;
use web_sys::MouseEvent;
#[derive(Clone, Copy, PartialEq)]
enum State {
    NotAdding,
    AddingWeekTemplate,
    AddingDayTemplate,
    AddingExercise,
}
#[component(inline_props)]
pub fn AddRoutine(routine: Signal<Option<Routine>>) -> View {
    // let template = create_signal(routine.with(|r|{r.as_ref().unwrap().templates().clone()}));
    fn default<T: Default>(dato: Signal<T>) {
        dato.set(T::default());
    }

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
        w.set(Vec::new());
        d.set(DayTemplate::default());
        e.set(Exercise::default());
        s.set(State::NotAdding);
    };

    let (tmp1, tmp2, tmp3) = (
        week_template.clone(),
        week_template.clone(),
        week_template.clone(),
    );

    let (dy_tmp, dy2, dy3, dy4) = (
        day_template.clone(),
        day_template.clone(),
        day_template.clone(),
        day_template.clone(),
    );
    create_memo(move ||{console_log!("{:#?}",dy4.get_clone())});
    let (s1, s2, s3, s4, s5, s6, s7) = (
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
        state.clone(),
    );
    let state_selector = create_selector(move || state.get());
    create_memo(move || console_log!("{:#?}", ex1.get_clone()));
    view! {
        (match state_selector.get(){
            State::NotAdding => view!{
                button(on:click =move |_|{
                    s1.set(State::AddingWeekTemplate);
                }){"Add Template"}
            },
            State::AddingWeekTemplate => {
                // let days_view = tmp3.get_clone_untracked().iter().enumerate().map(|(i,day)|view!{
                //     article(){
                //         (format!("Day {}, {} exercises",i,day.exercises().len()))
                //     }
                // }).collect::<Vec<View>>(); TODO!
                view!{
                    (days_view)
                    button(on:click =move |_|{
                        s6.set(State::AddingDayTemplate)
                    }){"Add Day"}
                    button(on:click= move |_|{
                        let rtn = Routine::build(None,week_template.get_clone(),[Week::default(),Week::default(),Week::default(),Week::default()],None,None,Some(String::from("Lucas")),Local::now().date_naive());
                        s6.set(State::NotAdding)
                    }){"Done"}
                    button(on:click = cancel ){"Cancel"}
                }
            },
            State::AddingDayTemplate => view!{
                button(on:click = move |_|{
                    s1.set(State::AddingExercise);
                }){"Add Exercise"}
                button(on:click = move |_| {
                    week_template.update(|w|{w.push(day_template.get_clone())});
                    s1.set(State::AddingWeekTemplate)
                }){"Done"}
                button(on:click = move |_|{
                    s5.set(State::AddingWeekTemplate)
                }){"Back"}
                button(on:click = cancel ){"Cancel"}
            },
            State::AddingExercise => view!{
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
