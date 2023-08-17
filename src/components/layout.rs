use crate::components::*;
use crate::routers::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props<R> where R: BetterRoute<R> {
    pub route: R,
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout<R>(props: &Props<R>) -> Html where R: BetterRoute<R> {
    let route = props.route.clone();

    html! {
        <>
            <Breadcrumbs<R> route={route} />
            <Navigation<R> />
            <hr />
            { for props.children.iter() }
        </>
    }
}
