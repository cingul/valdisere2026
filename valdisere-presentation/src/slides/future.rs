use dioxus::prelude::*;

#[component]
pub fn Future() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-brand-dark text-brand-light p-12 relative overflow-hidden",

             div { class: "z-10 mb-12 animate-fade-in-down",
                 h1 { class: "text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-4",
                    "Future Directions"
                }
                div { class: "h-1 w-32 bg-brand-orange rounded-full" }
            }

            div { class: "grid grid-cols-2 gap-12",
                div { class: "p-8 bg-brand-green/20 rounded-3xl border border-brand-green/50 animate-fade-in-left",
                    h3 { class: "text-2xl font-bold text-brand-orange mb-4", "RTC Design" }
                    p { class: "text-lg text-brand-light", "Moving towards a Randomized Control Trial to establish standard of care." }
                }

                div { class: "p-8 bg-brand-green/20 rounded-3xl border border-brand-green/50 animate-fade-in-right delay-200",
                    h3 { class: "text-2xl font-bold text-brand-orange mb-4", "Sham Control" }
                    p { class: "text-lg text-brand-light", "Implementing sham procedures to isolate the placebo effect in subjective symptom reporting." }
                }
            }
        }
    }
}
