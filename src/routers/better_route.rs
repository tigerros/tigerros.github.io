use yew::Html;
use yew_router::Routable;

pub trait BetterRoute<R>: Routable + 'static where R: BetterRoute<R> {
    fn switch(routes: R) -> Html;
    fn is_public(&self) -> bool;
    fn description(&self) -> &str;
}