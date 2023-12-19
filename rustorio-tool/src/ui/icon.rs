use yew::{
    function_component,
    html,
    use_context,
    AttrValue,
    Classes,
    Html,
    Properties,
};

use crate::AppData;

#[derive(PartialEq, Properties)]
pub struct IconProps {
    pub file_name: String,
    pub alt: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    pub size: Option<AttrValue>,
}

#[function_component]
pub fn FactorioIcon(
    IconProps {
        file_name,
        alt,
        class,
        size,
    }: &IconProps,
) -> Html {
    let data = use_context::<AppData>().expect("no ctx found");

    let src = data
        .game_data()
        .icon(file_name)
        .map(|s| format!("/data/icons/{s}"));

    html! { <img {src} {alt} class={class.clone()} width={size} /> }
}
