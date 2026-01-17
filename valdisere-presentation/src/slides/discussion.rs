use dioxus::prelude::*;

#[component]
pub fn Discussion() -> Element {
    rsx! {
        div {
             class: "flex flex-col h-full w-full bg-brand-dark text-brand-light p-12 relative overflow-hidden",

             div { class: "z-10 mb-12 animate-fade-in-down",
                 h1 { class: "text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-4",
                    "Discussion"
                }
                div { class: "h-1 w-32 bg-brand-orange rounded-full" }
            }

            div { class: "space-y-8 max-w-4xl animate-fade-in-up",
                div { class: "flex gap-6",
                    div { class: "w-2 h-full bg-brand-orange rounded-full" }
                    div {
                        h3 { class: "text-2xl font-bold text-brand-light mb-2", "Objective Validation" }
                        p { class: "text-xl text-brand-taupe", "The data confirms that alleviating venous obstruction correlates with improved orthostatic stability." }
                    }
                }

                 div { class: "flex gap-6",
                    div { class: "w-2 h-full bg-brand-green rounded-full" }
                    div {
                        h3 { class: "text-2xl font-bold text-brand-light mb-2", "Mechanical vs Autonomic" }
                        p { class: "text-xl text-brand-taupe", "Differentiating mechanical obstruction from pure autonomic failure is crucial for patient selection." }
                    }
                }
            }
        }
    }
}
