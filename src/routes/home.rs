use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <p>{"Welcome to my ("}<a href="https://www.github.com/tigerros">{"me"}</a>{") personal website! \
            There's not much here, but I figured a website would be nice to have. \
            Especially since it's free :) Courtesy of GitHub Pages!"}</p>
            <h1>{"About me"}</h1>
            <ol>
                <li>{"I'm a high school student from Prague."}</li>
                <li>{"I can really appreciate simplicity, which is why this website looks the way it looks. It may be basic, but I love it."}</li>
                <li>{"And I hate weakly typed languages, such as JavaScript. That's why I took the time to make this using Rust + WASM + "}
                <a href="https://yew.rs">{"Yew"}</a>{". The learning curve is tough, but the result is totally worth it."}</li>
            </ol>
        </>
    }
}
