use crate::router::Route;
use gloo_console::info;
use yew::prelude::*;
use yew_router::Routable;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub current_route: Route,
}

#[function_component(Breadcrumbs)]
pub fn breadcrumbs(props: &Props) -> Html {
    let current_path = props.current_route.clone().to_path().split_off(1);

    if current_path.is_empty() {
        return html! {
            <nav>
                <ol class="crumbs">
                    <li class="crumb"><a href={Route::Home.to_path()}>{"tigerros"}</a></li>
                </ol>
            </nav>
        };
    }

    let segments: Vec<&str> = current_path.split('/').collect();
    let segment_list: Html = segments
        .iter()
        .map(|&segment| {
            let segment_owned = segment.to_owned();

            html! {
                <li class="crumb">
                    <span class="pre">{" > "}</span>
                    <a href={segment_owned}>{segment}</a>
                </li>
            }
        })
        .collect();

    html! {
        <nav>
            <ol class="crumbs">
                <li class="crumb"><a href={Route::Home.to_path()}>{"tigerros"}</a></li>
                {segment_list}
            </ol>
        </nav>
    }
}
