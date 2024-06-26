use yew::html;
use crate::component::GitProjectList;

pub struct GitRepos;

impl yew::Component for GitRepos {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {<>
            <p>
            { "Here lies my git repositories ordered by most recently updated" } 
            <GitProjectList />
            </p>
        </>}
    }
}
