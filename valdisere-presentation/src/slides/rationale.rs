use dioxus::prelude::*;

#[component]
pub fn Rationale() -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-full w-full bg-brand-dark text-brand-light p-8",

            div { class: "z-10 mb-8 animate-fade-in-down",
                 h1 { class: "text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-4",
                    "Scientific Rationale"
                }
                div { class: "h-1 w-32 bg-brand-orange rounded-full" }
            }

            div { class: "grid grid-cols-2 gap-8 h-full animate-fade-in-up delay-300",

                // Left Column: Core Regulations
                div { class: "space-y-6",
                    div { class: "p-6 bg-brand-green/10 rounded-xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors",
                        h3 { class: "text-xl font-semibold text-brand-orange mb-2", "1. Baroreflex & Metabolite Clearance" },
                        p { class: "text-brand-taupe text-sm leading-relaxed",
                            "Efficiency depends on proper metabolite clearance (Fick Principle). Impaired venous outflow disrupts this, exacerbating OH."
                        }
                    }
                     div { class: "p-6 bg-brand-green/10 rounded-xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors",
                        h3 { class: "text-xl font-semibold text-brand-orange mb-2", "2. Venous Return Dynamics" },
                        p { class: "text-brand-taupe text-sm leading-relaxed",
                             "Gravity causes pooling (>500ml). Venous stenosis impedes return, dropping Cardiac Output >20%."
                        }
                    }
                     div { class: "p-6 bg-brand-green/10 rounded-xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors",
                        h3 { class: "text-xl font-semibold text-brand-orange mb-2", "3. Static Venous Tone" },
                        p { class: "text-brand-taupe text-sm leading-relaxed",
                             "Venous tone fails to adjust: Excessive supine constriction (Hypertension) vs Inadequate standing constriction (Hypotension)."
                        }
                    }
                }

                // Right Column: Advanced Mechanisms
                div { class: "space-y-6",
                    div { class: "p-6 bg-brand-green/10 rounded-xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors",
                         h3 { class: "text-xl font-semibold text-brand-orange mb-2", "4. Vestibulo-Sympathetic Reflex" },
                         p { class: "text-brand-taupe text-sm leading-relaxed",
                             "Venous congestion impairs key sympathetic pathways, leading to 'Sympathetic Anticipation Failure' prior to standing."
                         }
                    }
                    div { class: "p-6 bg-gradient-to-br from-brand-green/20 to-brand-orange/10 rounded-xl border border-brand-orange/40 shadow-lg transform scale-105",
                        h3 { class: "text-xl font-bold text-brand-orange mb-2", "5. Glymphatic Clearance" },
                         p { class: "text-brand-light text-sm leading-relaxed",
                             "Venous engorgement constricts perivenous spaces, impeding interstitial fluid drainage. Links venous obstruction to neurotoxic metabolite accumulation in autonomic centers."
                        }
                    }
                }
            }
        }
    }
}
