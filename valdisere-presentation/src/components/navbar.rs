use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        nav { class: "fixed top-0 left-0 right-0 h-16 bg-zinc-950/90 backdrop-blur-md border-b border-white/10 flex items-center justify-between px-8 z-50 transition-all",
            // Brand / Logo Area
            Link { to: Route::Intro {}, class: "font-black text-xl tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-red-500 hover:brightness-125 transition-all select-none",
                "STANDUP II"
            }

            // Navigation Links
            div { class: "flex items-center gap-1",
                NavLink { to: Route::Intro {}, label: "Home", active: current_route == Route::Intro {} }
                NavLink { to: Route::Motivation {}, label: "Motivation", active: current_route == Route::Motivation {} }
                NavLink { to: Route::Background {}, label: "Background", active: current_route == Route::Background {} }
                NavLink { to: Route::Rationale {}, label: "Rationale", active: current_route == Route::Rationale {} }
                NavLink { to: Route::Methods {}, label: "Methods", active: current_route == Route::Methods {} }
                NavLink { to: Route::Results {}, label: "Results", active: current_route == Route::Results {} }
                NavLink { to: Route::Discussion {}, label: "Discussion", active: current_route == Route::Discussion {} }
                NavLink { to: Route::CaseVideo {}, label: "Case Video", active: current_route == Route::CaseVideo {} }
            }

            // Right Side (Optional)
            div { class: "w-8 h-8 rounded-full bg-brand-orange/20 border border-brand-orange/50 flex items-center justify-center text-xs font-bold text-brand-orange",
                "S2"
            }
        }
    }
}

#[component]
fn NavLink(to: Route, label: &'static str, active: bool) -> Element {
    let base_class = "px-4 py-2 rounded-lg text-sm font-medium transition-all hover:bg-white/5";
    let active_class = "text-brand-orange bg-brand-orange/10";
    let inactive_class = "text-zinc-400 hover:text-white";

    let state_class = if active { active_class } else { inactive_class };

    rsx! {
        Link {
            to: to,
            class: "{base_class} {state_class}",
            "{label}"
        }
    }
}
