#![allow(non_snake_case)]

use crate::prelude::*;

#[inline_props]
pub fn Dirix(cx: Scope) -> Element {
    render! {
        p { "This page is in development." }
        p { "Links: " }
        ul {
            li { a { href: "https://github.com/tigerros/dirix", "GitHub" } }
            li { a { href: "https://crates.io/crates/dirix", "crates.io" } }
        }
    }
}