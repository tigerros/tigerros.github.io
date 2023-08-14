use crate::router::Route;
use gloo_console::info;
use yew_router::Routable;

mod components;
mod router;
mod routes;
mod web_util;

fn main() {
    yew::Renderer::<routes::App>::new().render();
}
