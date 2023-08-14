use crate::router::BetterRoute;
use gloo_console::info;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navigation)]
pub fn navigation<R>() -> Html
where
    R: BetterRoute,
{
    let routes = R::routes();

    let routes_table_rows: Html = routes
        .iter()
        .filter_map(|route_name| {
            // This will not recognize paths with parameters, eg. /person/:id
            let route = R::recognize(route_name).unwrap();

            if route.is_public() {
                let route_description = route.description().to_owned();

                Some(html! {
                    <tr>
                        <td><Link<R> to={route}>{route_name}</Link<R>></td>
                        <td>{route_description}</td>
                    </tr>
                })
            } else {
                None
            }
        })
        .collect();

    html! {
        <table>
            <thead>
                <tr>
                    <th>{"URL"}</th>
                    <th>{"Description"}</th>
                </tr>
            </thead>
            <tbody>{routes_table_rows}</tbody>
        </table>
    }
}
