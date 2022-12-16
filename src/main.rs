use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let play = use_state(|| false);
    let onclick = {
        let play = play.clone();
        move |_| {
            play.set(!*play);
        }
    };
    
    html! {
        <div styles="color: red;">
            <button {onclick}>{ "play" }</button>
            <p>{ *play }</p>
        </div>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
