use yew::{
    html,
    Html,
};
use yew_router::Routable;

use crate::ui::pages;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/research")]
    Research,
    #[at("/production")]
    Production,
    #[at("/blueprints")]
    Blueprints,
}

impl Route {
    pub fn render(&self) -> Html {
        match self {
            Route::Home => html! { <pages::home::Home /> },
            Route::Research => html! { <pages::research::Research /> },
            _ => html! {},
        }
    }
}
