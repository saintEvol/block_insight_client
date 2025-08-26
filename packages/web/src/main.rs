use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus::router::RouterConfig;
use models::app::app_state::AppState;
use models::state_provider::StateProvider;
use models::{init, init_network};
use ui::Navbar;
use ui::{
    Help,
    auth::{login::Login, register::Register},
    home::Home,
    modal::Modal,
};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[route("/:..routes")]
        Help { routes: Vec<String> },

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    console_error_panic_hook::set_once();
    init_network();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    info!("启动app");
    init();
    let app = AppState::use_context();
    let doing = app.doing.read_unchecked().clone();
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {
            // config: || {
            //     RouterConfig::default().on_update(|state|{
            //         state.current()
            //
            //         None
            //     })
            // }
        }
        match doing {
            None => {
                rsx!{}
            }
            Some(c) => {
                rsx!{
                    Modal{
                        content: c,
                    }
                }
            }
        }
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        // Navbar {
        //     Link {
        //         to: Route::Home {},
        //         "Home"
        //     }
        // }

        Outlet::<Route> {}
    }
}
