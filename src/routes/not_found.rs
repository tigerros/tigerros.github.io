#![allow(non_snake_case)]
use crate::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        h1 { "404" }
        p {
            "Not found. How about going back "
            Link { to: Route::Home {}, "home" }
            "?"
        }
        pre { "Attempted to navigate to: {segments.len():?}" }
    }
}