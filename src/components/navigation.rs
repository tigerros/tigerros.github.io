#![allow(non_snake_case)]
use crate::prelude::*;

#[inline_props]
pub fn Navigation(cx: Scope) -> Element {
    render! {
        nav {
            Link { to: Route::Home {}, "Home" }
            " "
            Link { to: Route::Contact {}, "Contact" }
        }
    }
}
