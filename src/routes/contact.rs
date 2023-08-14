use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <ul>
            <li><a href="https://github.com/tigerros">{"GitHub"}</a></li>
            <li><a href="mailto:tigerros.gh@gmail.com">{"Email"}</a></li>
            <li><a href="www.linkedin.com/in/aurel-leonard-danel-195470232">{"LinkedIn"}</a></li>
        </ul>
    }
}
