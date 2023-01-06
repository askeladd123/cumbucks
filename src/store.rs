use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Msg {
    BoughtCommon,
    BoughtRare,
    BoughtLegend,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub bux: u32,
    pub sock_bought: Callback<Msg>,
}

#[function_component(Store)]
pub fn store(props: &Props) -> Html {
    use crate::sock::price::*;

    let bux = props.bux;
    let sock_bought = props.sock_bought.clone();
    let onclick_common = move |_| {
        if COMMON <= bux {
            sock_bought.emit(Msg::BoughtCommon)
        }
    };

    let bux = props.bux;
    let sock_bought = props.sock_bought.clone();
    let onclick_rare = move |_| {
        if RARE <= bux {
            sock_bought.emit(Msg::BoughtRare)
        }
    };

    let bux = props.bux;
    let sock_bought = props.sock_bought.clone();
    let onclick_legend = move |_| {
        if LEGEND <= bux {
            sock_bought.emit(Msg::BoughtLegend)
        }
    };

    html! {
        <>
            <div style="text-align: center;" id="store">
                <button onclick={onclick_common} style={if props.bux < COMMON {"opacity: 50%;"} else {""}}>
                    <p class="sock_emoji">{"🧦"}</p>
                    <p class="sock_price">{format!("{}$", COMMON)}</p>
                </button>
                <button onclick={onclick_rare} style={if props.bux < RARE {"opacity: 50%;"} else {""}}>
                    <p class="sock_emoji">{"🧦"}</p>
                    <p class="sock_price">{format!("{}$", RARE)}</p>
                </button>
                <button onclick={onclick_legend} style={if props.bux < LEGEND {"opacity: 50%;"} else {""}}>
                    <p class="sock_emoji">{"🧦"}</p>
                    <p class="sock_price">{format!("{}$", LEGEND)}</p>
                </button>
            </div>
        </>
    }
}
