use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html{
    let tasks = vec!["one", "two", "three"];
    html!{
        <>
            <h1>{"hello world!"}</h1>
            <u1>
                {tasks.iter()
                    .map(|task| html!(<li>{task}</li>))
                    .collect::<Html>()}
            </u1>
        </>
    }

    

}