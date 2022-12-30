use gloo::console;
use yew::prelude::*;

pub struct Plan {
    open_tab: TabID,
}

#[derive(Properties, PartialEq)]
pub struct Props {}

impl Component for Plan {
    type Message = TabID;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            open_tab: TabID::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.open_tab = if self.open_tab == msg {
            TabID::None
        } else {
            msg
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_id = ctx.link().callback(|id| id);

        html! {
            <>
                <Tab
                    me={TabID::WorkEasy}
                    open={self.open_tab}
                    set_id={set_id.clone()}
                />
                <Tab
                    me={TabID::WorkHard}
                    open={self.open_tab}
                    set_id={set_id.clone()}
                />
            </>
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TabID {
    WorkHard,
    WorkEasy,
    TaskHard,
    TaskEasy,
    RewardSmall,
    RewardBig,
    None,
}

#[derive(Properties, PartialEq)]
struct TabProps {
    open: TabID,
    me: TabID,
    set_id: Callback<TabID>,
}

#[function_component(Tab)]
fn tab(props: &TabProps) -> Html {
    let set_id = props.set_id.clone();
    let me = props.me.clone();
    let onclick = move |_| set_id.emit(me);

    html! {
        <div class="tab">
            <button {onclick}>{format!("{:?}", props.me)}</button>
            if props.me == props.open {<p>{"funny shit"}</p>}
        </div>
    }
}
