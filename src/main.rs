pub mod app;
pub mod routes;
pub mod pages;
pub mod services;
pub mod error;

use app::App;

fn main() {
    yew::start_app::<App>();
}
