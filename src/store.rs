use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Msg{
    BoughtCommon,
    BoughtRare,
    BoughtLegend,
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub bux: u32,
    pub sock_bought: Callback<Msg>,
}

#[function_component(Store)]
pub fn store(props: &Props) -> Html{
    
    let style_emoji = "margin: 2px;";
    let style_price = "font-size: 18px; margin: 2px;";
    
    use crate::sock::price::*;
    
    let sock_bought = props.sock_bought.clone();
    let onclick_common = move|_|sock_bought.emit(Msg::BoughtCommon);
    
    let sock_bought = props.sock_bought.clone();
    let onclick_rare = move|_|sock_bought.emit(Msg::BoughtRare);
    
    let sock_bought = props.sock_bought.clone();
    let onclick_legend = move|_|sock_bought.emit(Msg::BoughtLegend);
    
    html!{
        <>
            <div style="text-align: center;" id="socks">
                <button onclick={onclick_common} style={if props.bux < COMMON {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>
                        {"🧦"}
                    </p>
                    <p style={style_price}>{format!("{}$", COMMON)}</p>
                </button>
                <button onclick={onclick_rare} style={if props.bux < RARE {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>{"🧦"}</p>
                    <p style={style_price}>{format!("{}$", RARE)}</p>
                </button>
                <button onclick={onclick_legend} style={if props.bux < LEGEND {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>{"🧦"}</p>
                    <p style={style_price}>{format!("{}$", LEGEND)}</p>
                </button>
            </div>
        </>
    }
}