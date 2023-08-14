mod components;
mod router;
mod routes;
mod web_util;

fn main() {
    yew::Renderer::<routes::App>::new().render();
}
