mod components;
mod routes;
mod web_util;
mod routers;

fn main() {
    yew::Renderer::<routes::App>::new().render();
}
