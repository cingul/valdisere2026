use dioxus::prelude::*;

#[component]
pub fn Physiology() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-zinc-950 text-white p-12 relative overflow-hidden",

            // Title
             div { class: "z-10 mb-12 animate-fade-in-down",
                h1 { class: "text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-500 mb-4",
                    "Physiological Mechanics"
                }
                div { class: "h-1 w-32 bg-emerald-500 rounded-full" }
            }

            div { class: "flex flex-grow items-center justify-center gap-24 z-10",

                // Diagram: Stenosed Vein
                div { class: "flex flex-col items-center space-y-4 animate-fade-in-up delay-200",
                    div { class: "relative w-32 h-96 bg-red-900/20 rounded-full border-4 border-red-500/30 overflow-hidden shadow-[0_0_30px_rgba(239,68,68,0.2)]",
                        // Narrowing
                        div { class: "absolute top-1/2 left-0 w-full h-8 bg-black/50 backdrop-blur-md scale-x-150 rotate-12" }
                        // Flow particles - Slow/Turbulent
                        for i in 0..5 {
                             div {
                                class: "absolute w-4 h-4 bg-red-500 rounded-full animate-bounce",
                                style: "left: 45%; top: {i * 20}%; animation-duration: {3.0 + i as f64 * 0.5}s;"
                             }
                        }
                    }
                    h3 { class: "text-xl font-semibold text-red-400", "Stenosis (Pre-Stent)" }
                    p { class: "text-zinc-400 text-sm max-w-[200px] text-center", "Turbulent flow, increased venous pressure, collaterals recruitment." }
                }

                // Arrow
                div { class: "text-4xl text-zinc-600 animate-pulse", "→" }

                // Diagram: Stented Vein
                div { class: "flex flex-col items-center space-y-4 animate-fade-in-up delay-400",
                     div { class: "relative w-32 h-96 bg-emerald-900/20 rounded-full border-4 border-emerald-500/30 overflow-hidden shadow-[0_0_30px_rgba(16,185,129,0.2)]",
                        // Stent Mesh (visual representation)
                        div { class: "absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4IiBoZWlnaHQ9IjgiPgo8cmVjdCB3aWR0aD0iOCIgaGVpZ2h0PSI4IiBmaWxsPSIjZmZmIiBmaWxsLW9wYWNpdHk9IjAuMSIvPgo8cGF0aCBkPSJNMCAwTDggOFo4IDBMMCA4IiBzdHJva2U9IiMxMGI5ODEiIHN0c9a2Utd2lkdGg9IjAuNSIvPjwvc3ZnPg==')] opacity-30" }

                        // Flow particles - Laminar/Fast
                        for i in 0..8 {
                             div {
                                class: "absolute w-4 h-4 bg-emerald-400 rounded-full animate-ping",
                                style: "left: 45%; top: {i * 12}%; animation-duration: 1s;"
                             }
                        }
                    }
                    h3 { class: "text-xl font-semibold text-emerald-400", "Restored Lumen (Post-Stent)" }
                    p { class: "text-zinc-400 text-sm max-w-[200px] text-center", "Laminar flow, pressure gradient normalization, symptom relief." }
                }
            }
        }
    }
}
