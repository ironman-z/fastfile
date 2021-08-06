pub mod app;
pub mod routes;
pub mod pages;

use app::App;

fn main() {
    yew::start_app::<App>();
}
