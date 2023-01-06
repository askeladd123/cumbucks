use gloo::console;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use yew::prelude::*;

use crate::plan::Instruction::WorkHard;
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
    // pub add_entry: Callback<(Instruction, String)>,
    // pub rm_entry: Callback<(Instruction, String)>,
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

        let set_id = ctx.link().callback(|id| id);

        use Instruction::*;

        // let add_entry = ctx.props().add_entry.clone();
        // let rm_entry = ctx.props().rm_entry.clone();
        let tabs = Instruction::iter()
            .map(|v| {
                // let add_entry = add_entry.clone();
                // let add_entry:Callback<_> = (move |val| add_entry.emit((v, val))).into();

                // let add_entry = add_entry.clone();
                // let rm_entry:Callback<_> = (move |val| rm_entry.emit((v, val))).into();

                html! {
                    <Tab
                        save={ctx.props().save.clone()}
                        me={v}
                        open={self.open_tab}
                        set_id={set_id.clone()}
                        // {add_entry}
                        // {rm_entry}
                        entries={ctx.props().instructions[&v].clone()}
                    />
                }
            })
            .collect::<Html>();

        html! {
            <>
                <button class="back" {onclick}>{"back"}</button>
                <div id="tabs">
                    {tabs}
                </div>
            </>
        }
    }
}

mod tab {
    use gloo::console;
    use gloo::storage::{LocalStorage, Storage};
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;
    use yew::prelude::*;

    use super::Instruction;

    pub struct Tab {
        values: HashSet<String>,
        name: String,
    }

    #[derive(Properties, PartialEq)]
    pub struct Props {
        pub open: Option<Instruction>,
        pub me: Instruction,
        pub set_id: Callback<Instruction>,
        pub entries: Rc<RefCell<HashSet<String>>>,
        pub save: Callback<()>,
        // pub add_entry: Callback<String>,
        // pub rm_entry: Callback<String>,
        // pub instructions: HashSet<String>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Msg {
        Add(String),
        Remove(String),
    }

    impl Component for Tab {
        type Message = Msg;
        type Properties = Props;

        fn create(ctx: &Context<Self>) -> Self {
            let name = ctx.props().me.to_string();
            let values = LocalStorage::get(&name).unwrap_or_else(|_| HashSet::new());

            Self { values, name }
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
            let set_id = ctx.props().set_id.clone();
            let me = ctx.props().me.clone();
            let onclick = move |_| set_id.emit(me);

            let entries = ctx.props().entries.clone();
            let link = ctx.link().clone();
            let onkeypress = move |e: KeyboardEvent| {
                if e.key() == "Enter" {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    let value = input.value();

                    link.send_message(Msg::Add(value));
                    // entries.borrow_mut().insert(value);

                    // add_entry.emit(value);

                    // link.send_message(Msg::Add(value));
                    input.set_value("");
                }
            };

            let link = ctx.link().clone();
            let list = ctx
                .props()
                .entries
                .clone()
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
                <div id={self.name.clone()} class="tab">
                    <button class="tab_button" {onclick}>{&self.name}</button>

                    if let Some(open) = ctx.props().open {
                        if ctx.props().me == open{
                            <div class="tab_content">
                                {list}
                                <input {onkeypress} placeholder="   add new item"/>
                            </div>
                        }
                    }
                </div>
            }
        }
    }
}
