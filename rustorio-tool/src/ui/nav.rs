use yew::{
    classes,
    function_component,
    html,
    html::ChildrenRenderer,
    Html,
    Properties,
};
use yew_router::{
    components::Link,
    hooks::use_route,
};

use crate::routes::Route;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavItemProps {
    pub children: ChildrenRenderer<Html>,
    pub route: Route,
}

#[function_component]
pub fn NavItem(props: &NavItemProps) -> Html {
    let is_active = use_route::<Route>()
        .map(|route| route == props.route)
        .unwrap_or_default();

    html! {
        <li class="nav-item">
            <Link<Route> to={props.route.clone()} classes={classes!("nav-link", is_active.then(|| Some("active")))}>
                {props.children.clone()}
            </Link<Route>>
        </li>
    }
}
