use dioxus::prelude::*;

#[component]
pub fn Methods() -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-full w-full bg-brand-dark text-brand-light p-8 relative",

             div { class: "z-10 mb-8 animate-fade-in-down",
                 h1 { class: "text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-2",
                    "Methodology (STANDUP 1)"
                }
                h2 { class: "text-xl text-brand-taupe",
                    "Diagnostic & Interventional Protocol"
                }
                div { class: "h-1 w-32 bg-brand-orange rounded-full mt-2" }
            }

            div { class: "grid grid-cols-3 gap-6 z-10 h-full pb-8",
                // Step 1: Diagnostics
                div { class: "flex flex-col bg-brand-green/10 p-6 rounded-2xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors animate-fade-in-up delay-100",
                    div { class: "text-5xl font-black text-brand-green/40 mb-4", "01" }
                    h3 { class: "text-xl font-bold text-brand-orange mb-4", "Comprehensive Diagnostics" }
                    ul { class: "space-y-3 text-sm text-brand-light",
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" }, "Cervical & Cranial Arteriography" }
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" }, "Venography (IJV, Subclavian, Brachiocephalic)" }
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" },
                            span { "Intravascular Ultrasound (IVUS):", br{}, span { class: "text-brand-taupe text-xs", "Sagittal, Transverse, Sigmoid Sinuses" } }
                        }
                    }
                }

                // Step 2: Intervention
                div { class: "flex flex-col bg-brand-green/10 p-6 rounded-2xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors animate-fade-in-up delay-300",
                    div { class: "text-5xl font-black text-brand-green/40 mb-4", "02" }
                    h3 { class: "text-xl font-bold text-brand-orange mb-4", "Venous Intervention" }
                    ul { class: "space-y-3 text-sm text-brand-light",
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" },
                            span { "Angioplasty (Based on sizing):", br{}, span { class: "text-brand-taupe text-xs", "Trek, Viatrac, Armada (up to 14mm)" } }
                        }
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" },
                            span { "Venous Stenting:", br{}, span { class: "text-brand-taupe text-xs", "Abre Stent (14-18mm x 60-120mm)" } }
                        }
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" }, "Post-Stent Angioplasty (10 atm)" }
                    }
                }

                // Step 3: Safety & Follow-up
                div { class: "flex flex-col bg-brand-green/10 p-6 rounded-2xl border border-brand-green/30 hover:border-brand-orange/50 transition-colors animate-fade-in-up delay-500",
                    div { class: "text-5xl font-black text-brand-green/40 mb-4", "03" }
                    h3 { class: "text-xl font-bold text-brand-orange mb-4", "Safety & Longitudinal Care" }
                     ul { class: "space-y-3 text-sm text-brand-light",
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" }, "Neuro Checks: q15min x 1hr post-op" }
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" }, "Adverse Event Monitoring & RCA" }
                        li { class: "flex items-start", span { class: "mr-2 text-brand-orange", "•" },
                            span { "Clinical Follow-up:", br{}, span { class: "text-brand-taupe text-xs", "2 weeks, 3mo, 6mo, 1yr, 2yr" } }
                        }
                    }
                }
            }
        }
    }
}
