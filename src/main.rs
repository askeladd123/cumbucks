#![allow(unused)]
use gloo::console;
use gloo::timers::callback::{Interval, Timeout};
use yew::platform::time::interval;
use yew::prelude::*;

mod log_in_out;
mod menu;
mod plan;
mod settings;
mod store;
mod unbox;
use log_in_out::LogInOut;
use menu::Menu;
use plan::Plan;
use settings::Settings;
use store::Store;
use unbox::Unbox;

pub mod sock {
    pub mod price {
        pub const COMMON: u32 = 3;
        pub const RARE: u32 = 50;
        pub const LEGEND: u32 = 200;
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

enum Msg {
    PPButton,
    Buck,
    Store(store::Msg),
    Menu(menu::Msg),
    Home,
}

enum Route {
    Home,
    Unbox,
    Settings,
    Plan,
    LogInOut,
}

struct App {
    bux: u32,
    counting: bool,
    in_store: bool,
    interval: Option<Interval>,
    route: Route,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            bux: 0,
            counting: false,
            in_store: false,
            interval: None,
            route: Route::Home,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PPButton => {
                self.counting = !self.counting;
                if self.counting {
                    let link = ctx.link().clone();
                    self.interval =
                        Some(Interval::new(1_000, move || link.send_message(Msg::Buck)));
                    self.in_store = false;
                }
            }
            Msg::Buck => {
                self.bux += 1;
                if self.bux % sock::price::COMMON == 0 {
                    self.in_store = true;
                    self.counting = false;
                }
            }
            Msg::Store(msg) => {
                self.route = Route::Unbox;
                self.bux -= match msg {
                    store::Msg::BoughtCommon => sock::price::COMMON,
                    store::Msg::BoughtRare => sock::price::RARE,
                    store::Msg::BoughtLegend => sock::price::LEGEND,
                };
                self.in_store = false;
                self.counting = false;
            }
            Msg::Menu(msg) => {
                self.route = match msg {
                    menu::Msg::Plan => Route::Plan,
                    menu::Msg::Settings => Route::Settings,
                    menu::Msg::LogInOut => Route::LogInOut,
                }
            }
            Msg::Home => self.route = Route::Home,
        }

        if !self.counting {
            self.interval = None;
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let go_back = ctx.link().callback(|_| Msg::Home);

        match self.route {
            Route::Home => html! {
                <>
                    <h1>{"do some work..."}</h1>
                    <div id="dashboard">
                        <p id="bux">{ self.bux }</p>
                        <button id="pp" onclick={ctx.link().callback(|_|Msg::PPButton)}>
                            {if self.counting{"⏸️"} else {"▶️"}}
                        </button>
                    </div>
                    if !self.in_store && !self.counting {
                        <Menu
                            set_route={ctx.link().callback(|msg|Msg::Menu(msg))}
                            logged_in={true}
                        />
                    }
                    if self.in_store {
                        <Store
                            bux={self.bux}
                            sock_bought={ctx.link().callback(|msg|Msg::Store(msg))}
                        />
                    }
                </>
            },
            Route::Unbox => html! {<><p>{"unbox"}</p></>},
            Route::Settings => html! {<Settings go_back={go_back.clone()}/>},
            Route::Plan => {
                html! {<Plan go_back={go_back.clone()}/>}
            }
            Route::LogInOut => html! {<LogInOut go_back={go_back.clone()}/>},
        }
    }
}
