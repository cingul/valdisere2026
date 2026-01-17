use dioxus::prelude::*;

#[component]
pub fn Standup2() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-zinc-950 text-white p-12 overflow-hidden relative",

            // Background effect
            div { class: "absolute top-0 right-0 w-1/2 h-full bg-gradient-to-l from-blue-900/10 to-transparent pointer-events-none" }

            div { class: "z-10 flex flex-col h-full justify-center space-y-12 max-w-4xl mx-auto animate-fade-in-up",
                div {
                    h1 { class: "text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-indigo-500 mb-6",
                        "Future: Standup 2"
                    }
                    div { class: "h-1 w-32 bg-blue-500 rounded-full" }
                }

                div { class: "space-y-8",
                    div { class: "p-8 bg-zinc-900/50 border-l-4 border-blue-500 rounded-r-xl backdrop-blur-sm",
                        h3 { class: "text-2xl font-semibold text-blue-300 mb-2", "Refined Protocol" }
                        p { class: "text-lg text-slate-300 leading-relaxed",
                            "Optimizing the orthostatic challenge for greater sensitivity and specificity. Introducing automated continuous monitoring."
                        }
                    }

                     div { class: "p-8 bg-zinc-900/50 border-l-4 border-indigo-500 rounded-r-xl backdrop-blur-sm",
                        h3 { class: "text-2xl font-semibold text-indigo-300 mb-2", "Expanded Metrics" }
                        p { class: "text-lg text-slate-300 leading-relaxed",
                            "Incorporating cerebral blood flow velocity (TCD) and near-infrared spectroscopy (NIRS) for comprehensive hemodynamic profiling."
                        }
                    }
                }
            }
        }
    }
}
