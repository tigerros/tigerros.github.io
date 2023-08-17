use yew::prelude::*;
use yew_router::prelude::*;
use super::BetterRoute;
use crate::components::*;
use crate::routers::main_router::NOT_FOUND_DESCRIPTION;
use crate::routers::MainRoute;
use crate::routes::*;

const HOME_DESCRIPTION: &str = "The projects home page.";
const DIRIX_DESCRIPTION: &str = "The Dirix project.";

#[derive(Clone, Routable, PartialEq)]
pub enum ProjectsRoute {
    #[at("/projects")]
    Home,
    #[not_found]
    #[at("/projects/404")]
    NotFound,
    #[at("/projects/dirix")]
    Dirix,
}

impl BetterRoute<ProjectsRoute> for ProjectsRoute {
    fn switch(routes: ProjectsRoute) -> Html {
        let route_html = match routes {
            ProjectsRoute::Home => html! { <Projects /> },
            ProjectsRoute::NotFound => html! { <Redirect<MainRoute> to={MainRoute::NotFound} /> },
            ProjectsRoute::Dirix => html! { <Dirix /> }
        };

        html! { <Layout<ProjectsRoute> route={routes}>{route_html}</Layout<ProjectsRoute>> }
    }

    /// Whether this route should be shown in the navigator.
    fn is_public(&self) -> bool {
        match &self {
            ProjectsRoute::Home => true,
            ProjectsRoute::NotFound => false,
            ProjectsRoute::Dirix => true,
        }
    }

    fn description(&self) -> &str {
        match &self {
            ProjectsRoute::Home => HOME_DESCRIPTION,
            ProjectsRoute::NotFound => NOT_FOUND_DESCRIPTION,
            ProjectsRoute::Dirix => DIRIX_DESCRIPTION,
        }
    }
}
