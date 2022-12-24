#![allow(unused)]
use yew::prelude::*;
use gloo::timers::callback::{Timeout, Interval};
use gloo::console;
use yew::platform::time::interval;

mod store;
use store::Store;

pub mod sock {
    pub mod price {
        pub const COMMON: u32 = 10;
        pub const RARE: u32 = 50;
        pub const LEGEND: u32 = 200;
    }
}

fn main() { yew::Renderer::<App>::new().render(); }

enum Msg{
    PPButton,
    Buck,
    Store(store::Msg),
}

struct App{
    bux: u32,
    counting: bool,
    interval: Option<Interval>,
}

impl Component for App{
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self{
            bux: 0,
            counting: false,
            interval: None,
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PPButton => {
                self.counting = !self.counting;
                if self.counting {
                    let link = ctx.link().clone();
                    self.interval = Some(Interval::new(
                        1_000,
                        move||link.send_message(Msg::Buck)
                    ));
                }
            }
            Msg::Buck => {
                self.bux += 1;
            }
            Msg::Store(msg) => {
                self.bux -= match msg {
                    store::Msg::BoughtCommon => sock::price::COMMON,
                    store::Msg::BoughtRare => sock::price::RARE,
                    store::Msg::BoughtLegend => sock::price::LEGEND,
                };
                self.counting = false;
            }
        }
        
        if !self.counting {
            self.interval = None;
        }
        
        true
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_|Msg::PPButton);
        let cb = ctx.link().callback(|msg|Msg::Store(msg));
        html! {
            <>
                <h1>{"do some work..."}</h1>
                <div id="dashboard">
                    <p>{ self.bux }</p>
                    <button {onclick}>{if self.counting{"⏸️"} else {"▶️"}}</button>
                </div>
                if sock::price::COMMON < self.bux {
                <div id="store">
                    <Store bux={self.bux} sock_bought={cb}/>
                </div>}
            </>
        }
    }
}