use dioxus::prelude::*;

#[component]
pub fn Intro() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-full w-full bg-brand-dark text-brand-light relative overflow-hidden",

            // Background Effects
            div { class: "absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none z-0",
                div { class: "absolute top-[-20%] left-[-10%] w-[50%] h-[50%] bg-brand-green/20 rounded-full blur-[100px] animate-pulse" }
                div { class: "absolute bottom-[-20%] right-[-10%] w-[50%] h-[50%] bg-brand-orange/10 rounded-full blur-[100px] animate-pulse delay-700" }
            }

            div {
                class: "flex flex-col items-center space-y-12 z-10 animate-fade-in-up",

                // Main Title
                h1 {
                    class: "text-8xl font-extrabold tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-brand-orange via-orange-500 to-amber-500 drop-shadow-[0_0_25px_rgba(240,87,8,0.3)]",
                    "The STANDUP Study"
                }

                // Subtitle / Event
                h2 {
                    class: "text-4xl font-light text-brand-taupe tracking-widest uppercase border-b border-brand-green pb-4",
                    "Venous Stenting in OH & Intolerance"
                }

                // Decorative Element
                div {
                    class: "flex space-x-2",
                    div { class: "w-3 h-3 rounded-full bg-brand-orange shadow-[0_0_10px_rgba(240,87,8,0.8)]" }
                    div { class: "w-3 h-3 rounded-full bg-brand-green shadow-[0_0_10px_rgba(40,62,40,0.8)]" }
                    div { class: "w-3 h-3 rounded-full bg-brand-taupe shadow-[0_0_10px_rgba(197,183,171,0.8)]" }
                }

                // Presenter / Context
                div {
                    class: "text-2xl text-brand-taupe/80 font-medium tracking-wide mt-12 text-center",
                    div { class: "font-bold text-brand-light mb-2", "Karthikeyan Arcot, MD" }
                    div { "Interventional Neurology" }
                    div { "Interventional Neuro Associates" }
                }

                // Logo
                 div {
                    class: "mt-8 p-6 bg-white/5 rounded-2xl backdrop-blur-sm border border-brand-green/20 hover:border-brand-green/40 transition-colors",
                    img {
                        src: "assets/ina-logo.png",
                        class: "h-32 object-contain filter drop-shadow-lg",
                        alt: "Interventional Neuro Associates"
                    }
                }
            }
        }
    }
}
