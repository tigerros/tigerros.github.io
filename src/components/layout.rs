#![allow(non_snake_case)]
use crate::prelude::*;

#[inline_props]
pub fn Layout(cx: Scope) -> Element {
    render! {
        header { Navigation {} }
        hr {}
        main { Outlet::<Route> {} }
    }
}
