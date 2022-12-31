use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub go_back: Callback<()>,
}

#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    let go_back = props.go_back.clone();
    let onclick = move |_| go_back.emit(());

    html! {
        <>
            <button {onclick}>{"back"}</button>
            <p>{"settings"}</p>
        </>
    }
}
