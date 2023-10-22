#![allow(non_snake_case)]
use crate::prelude::*;

pub fn Contact(cx: Scope) -> Element {
    render! {
        ul {
            li { a { href: "https://github.com/tigerros", "GitHub" } }
            li { a { href: "mailto:tigerros.gh@gmail.com", "Email" } }
            li { a { href: "https://www.linkedin.com/in/aurel-leonard-danel-195470232", "LinkedIn" } }
        }
    }
}