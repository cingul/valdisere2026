#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
pub mod slides;

// Use built-in Asset system if relevant, or just use string paths for simplicty in Dioxus 0.6+
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Intro {},
        #[route("/motivation")]
        Motivation {},
        #[route("/background")]
        Background {},
        #[route("/rationale")]
        Rationale {},
        #[route("/methods")]
        Methods {},
        #[route("/results")]
        Results {},
        #[route("/discussion")]
        Discussion {},
        #[route("/future")]
        Future {},
        #[route("/video")]
        CaseVideo {},
    #[end_layout]
    // 404 handler
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Inject Tailwind CSS
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        div { class: "min-h-screen w-full bg-zinc-950 text-slate-200 font-sans selection:bg-brand-orange/30 selection:text-brand-orange flex flex-col",
            // Navigation
            components::navbar::NavBar {}

            // Main Content Area
            // pt-16 accounts for the fixed navbar height
            main { class: "pt-16 flex-1 flex flex-col",
                Outlet::<Route> {}
            }
        }
    }
}

// Temporary placeholders for slides (will be moved to modules)
use crate::slides::background::Background;
use crate::slides::discussion::Discussion;
use crate::slides::future::Future;
use crate::slides::intro::Intro;
use crate::slides::methods::Methods;
use crate::slides::motivation::Motivation;
use crate::slides::rationale::Rationale;
use crate::slides::results::Results;
use crate::slides::video::CaseVideo;

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "h-screen flex items-center justify-center",
            div { class: "text-center",
                h1 { class: "text-4xl font-bold text-brand-orange mb-4", "404" }
                p { class: "text-zinc-400", "Page not found" }
                Link { to: Route::Intro {}, class: "mt-8 inline-block px-6 py-2 bg-brand-orange text-brand-dark rounded-full font-bold", "Go Home" }
            }
        }
    }
}
