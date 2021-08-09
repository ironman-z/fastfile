use yew::{html, Html};
use yew_router::prelude::*;

use crate::pages::{home::Home, login::Login};

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/404"]
    NotFound,
    #[to = "/"]
    Home,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
        AppRoute::Login => html! {<Login />},
        AppRoute::NotFound => html! {<h2>{"404"}</h2>},
    }
}
