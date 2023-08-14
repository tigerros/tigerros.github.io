use crate::router::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <h1>{"404"}</h1>
            <p>{"Not found. How about going back to "}<Link<Route> to={Route::Home}>{"home sweet home"}</Link<Route>>{"?"}</p>
        </>
    }
}
