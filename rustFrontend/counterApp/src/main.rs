use yew::prelude::*;

enum Msg {
    AddOne,
    SubtractOne,
    Reset,
}
struct CounterComp {
    count: i64,
}

impl Component for CounterComp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true //re-render component
            }
            Msg::SubtractOne => {
                self.count -= 1;
                true
            }
            Msg::Reset => {
                self.count = 0;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="conainer">
          <p>{self.count} </p>
          <div>
          <button onclick={link.callback(|_|Msg::AddOne)}>{"increment"}</button>
          <button onclick={link.callback(|_|Msg::SubtractOne)}>{"decrement"}</button>
          <button onclick={link.callback(|_|Msg::Reset)}>{"reset"}</button>
          </div>
           </div>
        }
    }
}

fn main() {
    yew::Renderer::<CounterComp>::new().render();
}
