use crate::routers::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props<R> where R: BetterRoute<R> {
    pub route: R,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs<R>(props: &Props<R>) -> Html where R: BetterRoute<R> {
    let current_path = props.route.clone().to_path().split_off(1);

    if current_path.is_empty() {
        return html! {
            <nav>
                <ol class="crumbs">
                    <li class="crumb"><Link<MainRoute> to={MainRoute::Home}>{"tigerros"}</Link<MainRoute>></li>
                </ol>
            </nav>
        };
    }

    let segments: Vec<&str> = current_path.split('/').collect();
    let segment_list: Html = segments
        .iter()
        .map(|&segment| {
            let segment_route = R::recognize(segment).unwrap();

            // <a href={segment_owned}>{segment}</a>
            html! {
                <li class="crumb">
                    <span class="pre">{" > "}</span>
                    <Link<R> to={segment_route}>{segment}</Link<R>>
                </li>
            }
        })
        .collect();

    html! {
        <nav>
            <ol class="crumbs">
                <li class="crumb"><a href={MainRoute::Home.to_path()}>{"tigerros"}</a></li>
                {segment_list}
            </ol>
        </nav>
    }
}
