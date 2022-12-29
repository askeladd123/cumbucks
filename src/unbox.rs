use yew::prelude::*;

pub struct Unbox {


}

impl Component for Unbox{
    type Message = ();
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"not ready bro"}</p>
            </>
        }
    }
}
