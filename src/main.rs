#![allow(unused)]

use gloo::storage::{LocalStorage, Storage};
use gloo::timers::callback::{Interval, Timeout};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

mod instructions;
mod log_in_out;
mod menu;
mod plan;
mod settings;
mod store;
mod unbox;

use instructions::Instruction;
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
    AddEntry { key: Instruction, val: String },
    RmEntry { key: Instruction, val: String },
    Save,
    SetCurrentInstruction(String),
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
    instructions: HashMap<Instruction, Rc<RefCell<HashSet<String>>>>,
    current_instruction: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut instructions = HashMap::new();
        for ins in Instruction::iter() {
            instructions.insert(
                ins,
                Rc::new(RefCell::new(
                    LocalStorage::get(ins.to_string()).unwrap_or_else(|_| HashSet::new()),
                )),
            );
        }

        Self {
            bux: 0,
            counting: false,
            in_store: false,
            interval: None,
            route: Route::Home,
            instructions,
            current_instruction: "do some work...".to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            PPButton => {
                self.counting = !self.counting;
                if self.counting {
                    let link = ctx.link().clone();
                    self.interval =
                        Some(Interval::new(1_000, move || link.send_message(Msg::Buck)));
                    self.in_store = false;
                }
            }
            Buck => {
                self.bux += 1;
                if self.bux % sock::price::COMMON == 0 {
                    self.in_store = true;
                    self.counting = false;
                }
            }
            Store(msg) => {
                self.route = Route::Unbox;
                self.bux -= match msg {
                    store::Msg::BoughtCommon => sock::price::COMMON,
                    store::Msg::BoughtRare => sock::price::RARE,
                    store::Msg::BoughtLegend => sock::price::LEGEND,
                };
                self.in_store = false;
                self.counting = false;
            }
            Menu(msg) => {
                self.route = match msg {
                    menu::Msg::Plan => Route::Plan,
                    menu::Msg::Settings => Route::Settings,
                    menu::Msg::LogInOut => Route::LogInOut,
                }
            }
            Home => self.route = Route::Home,
            AddEntry { key, val } => todo!(),
            RmEntry { key, val } => todo!(),
            Save => {
                for (k, v) in self.instructions.iter() {
                    LocalStorage::set(k.to_string(), v.clone().as_ref().borrow().deref())
                        .expect("Couldn't save elements in row: {k}.");
                }
            }
            SetCurrentInstruction(str) => {
                self.current_instruction = str;
            }
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
                    <h1>{&self.current_instruction}</h1>
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
            Route::Unbox => {
                html! {<Unbox 
                    go_back={go_back.clone()} 
                    instructions={self.instructions.clone()}
                    set_ins={ctx.link().callback(|str|Msg::SetCurrentInstruction(str))}
                    />}
            }
            Route::Settings => html! {<Settings go_back={go_back.clone()}/>},
            Route::Plan => {
                html! {<Plan
                save={ctx.link().callback(|_|Msg::Save)}
                go_back={go_back.clone()}
                instructions={self.instructions.clone()}
                />}
            }
            Route::LogInOut => html! {<LogInOut go_back={go_back.clone()}/>},
        }
    }
}
