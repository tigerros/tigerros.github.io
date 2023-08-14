use yew::platform::spawn_local;
use yew::prelude::*;

use crate::web_util::http_get_text;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub owner: AttrValue,
    pub repo: AttrValue,
    #[prop_or(AttrValue::from(""))]
    pub path: AttrValue,
}

#[function_component(GitHubDirListing)]
pub fn github_dir_listing(props: &Props) -> Html {
    let owner = props.owner.clone();
    let repo = props.repo.clone();
    let path = props.path.clone();
    let response = use_state_eq(|| String::new());
    let response2 = response.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local({
                async move {
                    response2.set(response_from_props_raw(&owner, &repo, &path).await);
                }
            });
        },
        (),
    );

    let response_raw = response.to_string();

    html! {
        <>{response_raw}</>
    }
}

async fn response_from_props_raw(owner: &AttrValue, repo: &AttrValue, path: &AttrValue) -> String {
    let contents_url = format!("https://api.github.com/repos/{owner}/{repo}/contents/{path}");

    http_get_text(&*contents_url).await
}
