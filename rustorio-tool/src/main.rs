mod data;
mod error;
mod logic;
mod routes;
mod ui;

use yew::{
    function_component,
    html,
    platform::spawn_local,
    use_effect,
    use_state,
    ContextProvider,
    Html,
};
use yew_router::{
    BrowserRouter,
    Routable,
    Switch,
};

use crate::{
    data::AppData,
    routes::Route,
    ui::{
        icon::FactorioIcon,
        nav::NavItem,
    },
};

#[function_component]
fn Loader() -> Html {
    let data = use_state(AppData::default);

    {
        let data = data.clone();
        use_effect(move || {
            if !data.is_loaded() {
                spawn_local(async move {
                    data.set(AppData::fetch().await.unwrap());
                });
            }
        });
    }

    if data.is_loaded() {
        html! {
            <ContextProvider<AppData> context={(*data).clone()}>
                <App/>
            </ContextProvider<AppData>>
        }
    }
    else {
        html! {
            <div class="d-flex justify-content-center flex-row flex-grow-1 w-100">
                <div class="spinner-border text-light" role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <BrowserRouter>
                <nav class="navbar navbar-dark bg-dark shadow-lg">
                    <div class="container-fluid">
                        <a class="navbar-brand" href="#">
                            <img src="/static/logo.png" alt="" height="24" class="d-inline-block align-text-top" />
                            <span class="navbar-brand-text">{"Rustorio"}</span>
                        </a>
                    </div>
                </nav>

                <div class="d-flex flex-row flex-grow-1">
                    <div class="d-flex flex-column flex-shrink-0 p-3 text-white bg-dark sidebar shadow-lg">
                        <ul class="nav nav-pills flex-column mb-auto">
                            <NavItem route={Route::Home}>
                                <yew_feather::Home class="nav-icon" />
                                {"Home"}
                            </NavItem>
                            <NavItem route={Route::Research}>
                                <FactorioIcon file_name="__base__/graphics/icons/lab.png" class="nav-icon" />
                                {"Research"}
                            </NavItem>
                            <NavItem route={Route::Production}>
                                <FactorioIcon file_name="__base__/graphics/icons/assembling-machine-3.png" class="nav-icon" />
                                {"Production"}
                            </NavItem>
                            <NavItem route={Route::Blueprints}>
                                <FactorioIcon file_name="__base__/graphics/icons/assembling-machine-3.png" class="nav-icon" />
                                {"Blueprints"}
                            </NavItem>
                        </ul>
                        <hr/>
                        <div class="dropdown">
                            <a href="#" class="d-flex align-items-center text-white text-decoration-none dropdown-toggle" id="dropdownUser1" data-bs-toggle="dropdown" aria-expanded="false">
                                <img src="/static/logo.png" alt="" width="32" height="32" class="rounded-circle me-2"/>
                                <strong>{"100%"}</strong>
                            </a>
                            <ul class="dropdown-menu dropdown-menu-dark text-small shadow" aria-labelledby="dropdownUser1">
                                <li>
                                    <a class="dropdown-item" href="#">{"New project"}</a>
                                </li>
                                <li>
                                    <a class="dropdown-item" href="#">{"Settings"}</a>
                                </li>
                            </ul>
                        </div>
                    </div>

                    <main>
                        <Switch<Route> render={|route: Route| route.render()} />
                    </main>
                </div>
            </BrowserRouter>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();

    let root = gloo_utils::document()
        .get_element_by_id("root")
        .expect("no root node found")
        .into();

    yew::Renderer::<Loader>::with_root(root).render();
}
