use prelude::*;

dry_mods::prelude! {
    mod pub use components, routes, route;
    pub use dioxus::prelude;
    pub use dioxus_router::prelude;
}

fn main() {
    dioxus_web::launch(App);
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}
