use yew::prelude::*;
use gloo::timers::callback::{Timeout, Interval};
use gloo::console;

enum Msg{
    Inc,
}

struct App{
    bux: u32,
    interval: Option<Interval>,
}

impl Component for App{
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        
        let link = ctx.link().clone();
        Self{
            bux: 0,
            interval: Some(Interval::new(1_000, move||link.send_message(Msg::Inc))),
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg {
            Msg::Inc => {
                if 10 <= self.bux{
                    self.interval = None
                } else {
                    self.bux += 1
                }
            }
        }
        
        true
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1 id="time">{ self.bux }</h1>
            </>
        }
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
