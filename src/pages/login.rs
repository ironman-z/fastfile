use yew::{Component, ComponentLink, html, Html, ShouldRender};

pub struct Login {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"Hello Login"}</h2>
            </div>
        }
    }
}
