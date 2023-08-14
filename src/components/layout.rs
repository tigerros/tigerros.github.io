use crate::components::*;
use crate::router::Route;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub route: Route,
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    let route = props.route.clone();

    html! {
        <>
            <Breadcrumbs current_route={route} />
            <Navigation<Route> />
            <hr />
            { for props.children.iter() }
        </>
    }
}
