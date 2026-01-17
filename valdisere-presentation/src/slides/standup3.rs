use dioxus::prelude::*;

#[component]
pub fn Standup3() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-zinc-950 text-white p-12 overflow-hidden relative",

            // Background effect
            div { class: "absolute bottom-0 left-0 w-1/2 h-full bg-gradient-to-tr from-purple-900/10 to-transparent pointer-events-none" }

            div { class: "z-10 flex flex-col h-full justify-center items-center text-center space-y-16 max-w-5xl mx-auto animate-fade-in-up",
                div {
                    h1 { class: "text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-500 mb-6",
                        "Future: Standup 3"
                    }
                    h2 { class: "text-3xl text-slate-400 font-light", "(Sham Controlled)" }
                }

                div { class: "grid grid-cols-2 gap-12 text-left",
                    div { class: "p-8 bg-zinc-900/50 rounded-2xl border border-zinc-800 hover:border-purple-500/50 transition-colors",
                        h3 { class: "text-xl font-bold text-purple-400 mb-4 uppercase tracking-wider", "The Challenge" }
                        p { class: "text-slate-300",
                            "Differentiating placebo effect from physiological benefit in venous stenting efficacy."
                        }
                    }

                     div { class: "p-8 bg-zinc-900/50 rounded-2xl border border-zinc-800 hover:border-pink-500/50 transition-colors",
                        h3 { class: "text-xl font-bold text-pink-400 mb-4 uppercase tracking-wider", "The Approach" }
                        p { class: "text-slate-300",
                            "Double-blinded randomized crossover design with sham procedure arm to validate stenting outcomes."
                        }
                    }
                }

                div { class: "text-8xl text-zinc-800 font-black tracking-tighter opacity-20 absolute bottom-12", "RCT" }
            }
        }
    }
}
