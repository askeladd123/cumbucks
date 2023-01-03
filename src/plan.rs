use gloo::console;
use yew::prelude::*;

use crate::plan::Instruction::WorkHard;
use crate::Instruction;
use tab::Tab;

pub struct Plan {
    open_tab: Instruction,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub go_back: Callback<()>,
}

impl Component for Plan {
    type Message = Instruction;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            open_tab: Instruction::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.open_tab = if self.open_tab == msg {
            Instruction::None
        } else {
            msg
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let go_back = ctx.props().go_back.clone();
        let onclick = move |_| go_back.emit(());

        let set_id = ctx.link().callback(|id| id);

        use Instruction::*;
        let tabs = [
            WorkHard,
            WorkEasy,
            TaskEasy,
            TaskHard,
            RewardSmall,
            RewardBig,
        ]
        .into_iter()
        .map(|v| {
            html! {
                <Tab
                    me={v}
                    open={self.open_tab}
                    set_id={set_id.clone()}
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
    use std::collections::HashSet;
    use yew::prelude::*;

    use super::Instruction;

    pub struct Tab {
        values: HashSet<String>,
        name: String,
    }

    #[derive(Properties, PartialEq)]
    pub struct Props {
        pub open: Instruction,
        pub me: Instruction,
        pub set_id: Callback<Instruction>,
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
            let name = format!("{:?}", ctx.props().me);
            let values = LocalStorage::get(&name).unwrap_or_else(|_| HashSet::new());

            Self { values, name }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::Add(v) => self.values.insert(v),
                Msg::Remove(v) => self.values.remove(&v),
            };
            LocalStorage::set(&self.name, &self.values).unwrap();
            true
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let set_id = ctx.props().set_id.clone();
            let me = ctx.props().me.clone();
            let onclick = move |_| set_id.emit(me);

            let stuff = vec!["hei der", "hello there", "hola alli"];

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
            let entries = self
                .values
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
                    if ctx.props().me == ctx.props().open {
                        <div class="tab_content">
                            {entries}
                            <input {onkeypress} placeholder="   add new item"/>
                        </div>
                    }
                </div>
            }
        }
    }
}
