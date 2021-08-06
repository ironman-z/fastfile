use yew::{html, Html};
use yew_router::prelude::*;

use crate::pages::{home::Home};

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Home,
}

pub fn app_switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<Home />},
    }
}
