use structs::exercise::{Exercise, MuscleGroup, Series};
use sycamore::prelude::*;
use web_sys::KeyboardEvent;

#[component(inline_props)]
pub fn AddExercise(exercise: Signal<Exercise>) -> View {
    let (ex0, ex5, ex6) = (exercise.clone(), exercise.clone(), exercise.clone());
    let name = create_signal(String::new());
    let series = [
        create_signal(String::new()),
        create_signal(String::new()),
        create_signal(String::new()),
        create_signal(String::new()),
    ];
    let inputs = series.iter().cloned().enumerate().map(|(i,s)|{view!{
        input(placeholder = format!("{}",match i{
            0=>"1st",
            1=>"2nd",
            2=>"3rd",
            _=>"4th",
        }),r#type = "number", bind:value = s, on:keydown = move |_:KeyboardEvent|{
            ex6.update(|ex|{
                let string = s.get_clone();
                ex.set_series_at(i,(string.len()>0).then(||{Series::build(None,string.parse().unwrap(),None)}));
            })
        }){}
    }}).collect::<Vec<View>>();
    let group = create_signal(String::new());
    let options = MuscleGroup::iter()
        .into_iter()
        .map(|m| {
            view! {option(value = m.to_string()){(m.to_string())}}
        })
        .collect::<Vec<View>>();
    view! {
        input(placeholder = "Name", bind:value = name, on:keyup = move |_:KeyboardEvent| {
            ex0.update(|ex|ex.set_name(name.get_clone()))
        }){}
        (inputs)
        select(bind:value = group, value= "Chest", on:change = move |_| {
            ex5.update(|ex|ex.set_group(MuscleGroup::try_from(group.get_clone()).unwrap()))
        }){
            option(value = "", disabled = true, hidden = true){"Select"}
            (options)
        }
    }
}
