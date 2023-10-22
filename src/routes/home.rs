use crate::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render! { p { "Hey! There's nothing here." } }
}
