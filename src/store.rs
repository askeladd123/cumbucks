use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub bux: u32,
}

#[function_component]
pub fn Store(props: &Props) -> Html{
    
    let style_emoji = "margin: 2px;";
    let style_price = "font-size: 18px; margin: 2px;";
    
    use crate::sock::price::*;
    
    html!{
        <>
            <div style="text-align: center;" id="socks">
                <button style={if props.bux < COMMON {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>
                        {"🧦"}
                    </p>
                    <p style={style_price}>{format!("{}$", COMMON)}</p>
                </button>
                <button style={if props.bux < RARE {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>{"🧦"}</p>
                    <p style={style_price}>{format!("{}$", RARE)}</p>
                </button>
                <button style={if props.bux < LEGEND {"opacity: 50%;"} else {""}}>
                    <p style={style_emoji}>{"🧦"}</p>
                    <p style={style_price}>{format!("{}$", LEGEND)}</p>
                </button>
            </div>
        </>
    }
}