use crate::instructions::Instruction;
use gloo::console;
use gloo::timers::callback::Timeout;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
use yew::prelude::*;

pub struct Unbox {
    instruction_type: Option<Instruction>,
    instruction: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub instructions: HashMap<Instruction, Rc<RefCell<HashSet<String>>>>,
    pub go_back: Callback<()>,
    pub set_ins: Callback<String>,
}

pub enum Msg {
    Opened1,
    Opened2,
}

impl Component for Unbox {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Timeout::new(1000, move || {
            link.send_message(Msg::Opened1);
            Timeout::new(2000, move || {
                link.send_message(Msg::Opened2);
            })
            .forget();
        })
        .forget();
        Self {
            instruction_type: None,
            instruction: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Opened1 => {
                self.instruction_type = Some(
                    ctx.props()
                        .instructions
                        .keys()
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .clone(),
                );
            }
            Msg::Opened2 => {
                if let Some(t) = self.instruction_type {
                    self.instruction = if let Some(t) = ctx.props().instructions[&t]
                        .as_ref()
                        .borrow()
                        .iter()
                        .choose(&mut rand::thread_rng())
                    {
                        Some(t.clone())
                    } else {
                        Some("got nothing".to_owned())
                    };
                    ctx.props().set_ins.emit(self.instruction.clone().unwrap());
                }

                let go_back = ctx.props().go_back.clone();
                Timeout::new(3000, move || go_back.emit(())).forget();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                if let Some(ref t) = self.instruction_type{ <p>{t.to_string()}</p> }
                if let Some(ref t) = self.instruction { <p>{t}</p> }
            </>
        }
    }
}
