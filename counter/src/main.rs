use yew::prelude::*;

fn main() {
    yew::start_app::<Counter>();
}

enum Change {
    Increment,
    Decrement,
}

struct Counter {
    value: i64,
}

impl Component for Counter {
    type Message = Change;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Change::Decrement => {
                self.value -= 1;
                true
            }
            Change::Increment => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!(
            <main>

                <div>
                    {self.value}
                </div>

                <button onclick={link.callback(|_| Change::Decrement)}>{"-"}</button>
                <button onclick={link.callback(|_| Change::Increment)}>{"+"}</button>
            </main>
        )
    }
}
