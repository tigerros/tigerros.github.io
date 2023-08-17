use yew::prelude::*;

#[function_component(Dirix)]
pub fn dirix() -> Html {
    html! {
        <>
            <p>{"This page is in development."}</p>
            <p>{"Links:"}</p>
            <ul>
                <li><a href="https://github.com/tigerros/dirix">{"GitHub"}</a></li>
                <li><a href="https://crates.io/crates/dirix">{"crates.io"}</a></li>
            </ul>
        </>
    }
}