use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::*;
use crate::routes::*;
use super::BetterRoute;
use super::ProjectsRoute;

const HOME_DESCRIPTION: &str = "The home page.";
pub(super) const NOT_FOUND_DESCRIPTION: &str = "Page for 404 requests.";
const CDN_DESCRIPTION: &str = "\"CDN\" listing of some files.";
const CONTACT_DESCRIPTION: &str = "How to contact me.";
const PROJECTS_DESCRIPTION: &str = "My best creations.";

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/cdn")]
    Cdn,
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    ProjectsRoot,
    #[at("/projects/*")]
    Projects,
}

impl BetterRoute<MainRoute> for MainRoute {
    fn switch(routes: MainRoute) -> Html {
        let route_html = match routes {
            MainRoute::Home => html! { <Home /> },
            MainRoute::NotFound => html! { <NotFound /> },
            MainRoute::Cdn => html! { <Cdn /> },
            MainRoute::Contact => html! { <Contact /> },
            MainRoute::ProjectsRoot | MainRoute::Projects => html! { <Switch<ProjectsRoute> render={ProjectsRoute::switch} /> },
        };

        if routes == MainRoute::ProjectsRoot || routes == MainRoute::Projects {
            route_html
        } else {
            html! { <Layout<MainRoute> route={routes}>{route_html}</Layout<MainRoute>> }
        }
    }

    /// Whether this route should be shown in the navigator.
    fn is_public(&self) -> bool {
        match &self {
            MainRoute::Home => true,
            MainRoute::NotFound => false,
            MainRoute::Cdn => false,
            MainRoute::Contact => true,
            MainRoute::ProjectsRoot => true,
            MainRoute::Projects => false,
        }
    }

    fn description(&self) -> &str {
        match &self {
            MainRoute::Home => HOME_DESCRIPTION,
            MainRoute::NotFound => NOT_FOUND_DESCRIPTION,
            MainRoute::Cdn => CDN_DESCRIPTION,
            MainRoute::Contact => CONTACT_DESCRIPTION,
            MainRoute::ProjectsRoot | MainRoute::Projects => PROJECTS_DESCRIPTION,
        }
    }
}
