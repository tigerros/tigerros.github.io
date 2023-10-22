use crate::prelude::*;

#[derive(Routable, Clone, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/contact")]
        Contact {},
        #[route("/blog")]
        Blog {},
        #[nest("/projects")]
            #[route("/")]
            Projects {},
            #[route("/dirix")]
            Dirix {},
        #[end_nest]
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> }
}
