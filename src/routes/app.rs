use yew::prelude::*;
use yew_router::prelude::*;

use crate::routers::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={MainRoute::switch} />
        </BrowserRouter>
    }
}
