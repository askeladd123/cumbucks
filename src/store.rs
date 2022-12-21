use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub bux: u32,
}

#[function_component]
pub fn Store(props: &Props) -> Html{
    
    let style_emoji = "margin: 2px;";
    let style_price = "font-size: 18px; margin: 2px;";
    
    let sock_price_common = 10;
    let sock_price_rare = 50;
    let sock_price_legend = 200;
    
    html!{
        <>
            <div style="text-align: center;" id="socks">
                <button style={if props.bux < sock_price_common {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>
                        {"🧦"}
                    </p>
                    <p style={style_price}>{format!("{}$", sock_price_common)}</p>
                </button>
                <button style={if props.bux < sock_price_rare {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>{"🧦"}</p>
                    <p style={style_price}>{format!("{}$", sock_price_rare)}</p>
                </button>
                <button style={if props.bux < sock_price_legend {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>{"🧦"}</p>
                    <p style={style_price}>{format!("{}$", sock_price_legend)}</p>
                </button>
            </div>
        </>
    }
}