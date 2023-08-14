use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::switch} />
        </BrowserRouter>
    }
}
