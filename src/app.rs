use yew::{Component, ComponentLink, Html, html, ShouldRender};
use yew_router::prelude::*;

use crate::routes::{AppRoute, switch};

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for App {
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
                <h1>{ "Hello App" }</h1>
                <Router<AppRoute> render={Router::render(switch)} />
            </div>    
        }
    }
}
