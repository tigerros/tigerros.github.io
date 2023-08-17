use crate::components::*;
use yew::prelude::*;

#[function_component(Cdn)]
pub fn cdn() -> Html {
    html! {
        <GitHubDirListing owner="tigerros" repo="tigerros" />
    }
}
