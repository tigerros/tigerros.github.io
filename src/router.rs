use yew::prelude::*;

use crate::components::*;
use yew_router::prelude::*;

use crate::routes::*;

const HOME_DESCRIPTION: &'static str = "The home page.";
const CDN_DESCRIPTION: &'static str = "\"CDN\" listing of some files.";
const CONTACT_DESCRIPTION: &'static str = "How to contact me.";
const NOT_FOUND_DESCRIPTION: &'static str = "Page for 404 requests.";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cdn")]
    Cdn,
    #[at("/contact")]
    Contact,
    #[at("/post/n")]
    Post,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub trait BetterRoute: Routable + 'static {
    fn switch(routes: Route) -> Html;
    fn is_public(&self) -> bool;
    fn description(&self) -> &str;
}

impl BetterRoute for Route {
    fn switch(routes: Route) -> Html {
        let html = match routes {
            Route::Home => html! { <Home /> },
            Route::Cdn => html! { <Cdn /> },
            Route::Contact => html! { <Contact /> },
            Route::NotFound => html! { <NotFound /> },
            Route::Post => html! { {"Post"} },
        };

        html! { <Layout route={routes}>{html}</Layout> }
    }

    /// Whether this route should be shown in the navigator.
    fn is_public(&self) -> bool {
        match &self {
            Route::Home => true,
            Route::Cdn => false,
            Route::Contact => true,
            Route::NotFound => false,
            Route::Post => true,
        }
    }

    fn description(&self) -> &str {
        match &self {
            Route::Home => HOME_DESCRIPTION,
            Route::Cdn => CDN_DESCRIPTION,
            Route::Contact => CONTACT_DESCRIPTION,
            Route::NotFound => NOT_FOUND_DESCRIPTION,
            Route::Post => "Path segment testing.",
        }
    }
}
