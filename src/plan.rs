use gloo::console;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use yew::prelude::*;

use crate::Instruction;
use tab::Tab;

pub struct Plan {
    open_tab: Option<Instruction>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub go_back: Callback<()>,
    pub instructions: HashMap<Instruction, Rc<RefCell<HashSet<String>>>>,
    pub save: Callback<()>,
}

impl Component for Plan {
    type Message = Instruction;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { open_tab: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.open_tab = if self.open_tab == Some(msg) {
            None
        } else {
            Some(msg)
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let go_back = ctx.props().go_back.clone();
        let save = ctx.props().save.clone();
        let onclick = move |_| go_back.emit(());

        let request_open_tab = ctx.link().callback(|id| id);

        let tabs = Instruction::iter()
            .map(|v| {
                html! {
                    <Tab
                        save={ctx.props().save.clone()}
                        tab={v}
                        open_tab={self.open_tab}
                        request_to_open_tab={request_open_tab.clone()}
                        entries={ctx.props().instructions[&v].clone()}
                    />
                }
            })
            .collect::<Html>();

        html! {
            <>
                <button class="back" {onclick}>{"back"}</button>
                <div id="tabs">{tabs}</div>
            </>
        }
    }
}

mod tab {
    use gloo::console;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;
    use yew::prelude::*;

    use super::Instruction;

    pub struct Tab;

    #[derive(Properties, PartialEq)]
    pub struct Props {
        pub entries: Rc<RefCell<HashSet<String>>>,
        pub tab: Instruction,
        pub open_tab: Option<Instruction>,
        pub request_to_open_tab: Callback<Instruction>,
        pub save: Callback<()>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Msg {
        Add(String),
        Remove(String),
    }

    impl Component for Tab {
        type Message = Msg;
        type Properties = Props;

        fn create(ctx: &Context<Self>) -> Self {
            Self
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::Add(v) => {
                    ctx.props().entries.as_ref().borrow_mut().insert(v);
                    ctx.props().save.emit(());
                }
                Msg::Remove(v) => {
                    ctx.props().entries.as_ref().borrow_mut().remove(&v);
                    ctx.props().save.emit(());
                }
            };
            true
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let tab = ctx.props().tab;
            let request_to_open_tab = ctx.props().request_to_open_tab.clone();
            let onclick = move |_| request_to_open_tab.emit(tab);

            let link = ctx.link().clone();
            let onkeypress = move |e: KeyboardEvent| {
                if e.key() == "Enter" {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    let value = input.value();

                    link.send_message(Msg::Add(value));
                    input.set_value("");
                }
            };

            let link = ctx.link().clone();
            let list = ctx
                .props()
                .entries
                .as_ref()
                .borrow()
                .clone()
                .into_iter()
                .map(|k| {
                    let link = link.clone();
                    html! {
                        <div key={k.clone()}>
                            <div class="entry">{k.clone()}</div>
                            <button onclick={
                                move|_|link.send_message(Msg::Remove(k.clone()))
                            }>{"-"}</button>
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div id={tab.to_string()} class="tab">
                    <button class="tab_button" {onclick}>{tab.to_string()}</button>

                    if let Some(open) = ctx.props().open_tab {
                        if ctx.props().tab == open{
                            <div class="tab_content">
                                {list}
                                <input {onkeypress} placeholder="\tadd new item"/>
                            </div>
                        }
                    }
                </div>
            }
        }
    }
}
