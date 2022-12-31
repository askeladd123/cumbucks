use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub go_back: Callback<()>,
}

#[function_component(LogInOut)]
pub fn log_in_out(props: &Props) -> Html {
    let go_back = props.go_back.clone();
    let onclick = move |_| go_back.emit(());

    html! {
        <>
            <button {onclick}>{"back"}</button>
            <p>{"log in/out"}</p>
        </>
    }
}
