use yew::{Component, ComponentLink, html, Html,  ShouldRender};

pub struct Home {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"Hello Home"}</h2>
            </div>
        }
    }
}
