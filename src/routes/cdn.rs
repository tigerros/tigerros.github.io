use crate::components::*;
use crate::router::*;
use yew::prelude::*;

#[function_component(Cdn)]
pub fn cdn() -> Html {
    html! {
        <GitHubDirListing owner="tigerros" repo="tigerros" />
    }
}
