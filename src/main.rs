#![allow(unused)]
use yew::prelude::*;
use gloo::timers::callback::{Timeout, Interval};
use gloo::console;
use yew::platform::time::interval;

mod store;
use store::Store;

fn main() { yew::Renderer::<App>::new().render(); }

enum Msg{
    PPButton,
    Buck,
}

struct App{
    bux: u32,
    interval: Option<Interval>,
    button_icon: String,
}

impl Component for App{
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self{
            bux: 0,
            interval: None,
            button_icon: "▶️".into(),
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg {
            Msg::PPButton => if self.interval.is_none(){
                self.button_icon = "⏸".into();
                let link = ctx.link().clone();
                self.interval = Some(
                    Interval::new(1_000, move||link.send_message(Msg::Buck))
                );
            } else {
                self.button_icon = "▶️".into();
                self.interval = None;
            }
            Msg::Buck => {
                self.bux += 1;
            }
        }
        
        true
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let link = ctx.link().clone();
        let pp_button = move|_|link.clone().send_message(Msg::PPButton);
        
        html! {
            <>
                <h1>{"do some work..."}</h1>
                <div class="bob">
                    <p>{ self.bux }</p>
                    <button onclick={pp_button}>{self.button_icon.clone()}</button>
                </div>
                <div id="store">
                    <Store bux={self.bux}/>
                </div>
            </>
        }
    }
}
