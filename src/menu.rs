use yew::prelude::*;

pub enum Msg {
    Plan,
    Settings,
    LogInOut,
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub set_route:Callback<Msg>,
    pub logged_in: bool,
}

#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {

    let set_route = props.set_route.clone();
    let onclick_plan = move|_|set_route.emit(Msg::Plan);

    let set_route = props.set_route.clone();
    let onclick_settings = move|_|set_route.emit(Msg::Settings);

    let set_route = props.set_route.clone();
    let onclick_log_in_out = move|_|set_route.emit(Msg::LogInOut);

    html!{
        <>
            <div id="menu">
                <button onclick={onclick_plan}>{"plan"}</button>
                <button onclick={onclick_settings}>{"settings"}</button>
                if props.logged_in {
                    <button onclick={onclick_log_in_out.clone()}>{"log out"}</button>
                } else {
                    <button onclick={onclick_log_in_out}>{"log in"}</button>
                }
            </div>
        </>
    }
}