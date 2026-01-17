use dioxus::prelude::*;

#[component]
pub fn Motivation() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-brand-dark text-brand-light p-12 relative overflow-hidden",

            div { class: "z-10 mb-12 animate-fade-in-down",
                h1 { class: "text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-4",
                    "Motivation"
                }
                div { class: "h-1 w-32 bg-brand-orange rounded-full" }
            }

            div { class: "grid grid-cols-2 gap-16 z-10",
                div { class: "space-y-8 animate-fade-in-left delay-200",
                    div { class: "p-8 bg-brand-green/30 rounded-3xl border border-brand-green/50 backdrop-blur-sm",
                        h3 { class: "text-2xl font-semibold text-brand-orange mb-4", "The Problem" }
                        p { class: "text-xl text-brand-light leading-relaxed",
                            "Orthostatic Hypotension (OH) remains a prevalent condition with significant morbidity. Current diagnostic approaches often rely on subjective patient reporting or transient hemodynamic markers."
                        }
                    }
                }

                div { class: "space-y-8 animate-fade-in-right delay-400",
                     div { class: "p-8 bg-brand-green/30 rounded-3xl border border-brand-green/50 backdrop-blur-sm",
                        h3 { class: "text-2xl font-semibold text-brand-orange mb-4", "The Need" }
                         p { class: "text-xl text-brand-light leading-relaxed",
                            "We need an **objective parameter** of venous outflow obstruction to guide intervention. Isolating the mechanical component of OH specific to venous return is critical for effective stenting."
                        }
                    }
                }
            }
        }
    }
}
