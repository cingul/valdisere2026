use dioxus::prelude::*;

#[component]
pub fn Background() -> Element {
    // Try both absolute and relative paths for debugging, but in Dioxus + Assets, usually "bradbury.png" if served from root or "/assets/bradbury.png"
    // Since we saw "assets/tailwind.css" works in main.rs, let's try explicit asset invocation or just assume standard serving
    // If "bradbury.png" is in "assets", and the server serves "assets", then "/assets/bradbury.png" is safest.

    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-brand-dark text-brand-light p-8 overflow-hidden items-center justify-center",

            // Header
            div { class: "z-10 mb-8 animate-fade-in-down text-center",
                h1 { class: "text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-4",
                    "Historical Context"
                }
                div { class: "h-1 w-32 bg-brand-orange rounded-full mx-auto" }
            }

            div { class: "grid grid-cols-2 gap-12 w-full max-w-6xl animate-fade-in-up delay-300",

                // LEFT: Bradbury Image
                div { class: "flex flex-col items-center justify-center bg-brand-green/5 rounded-2xl p-8 border border-brand-green/20",
                     // Using straight asset path. Dioxus Fullstack usually serves public or assets folder at root.
                     // The user file listing shows `assets/bradbury.png`.
                     // Let's try `assets/bradbury.png` assuming simple file serving.
                    img {
                        src: "assets/bradbury.png",
                        class: "rounded-lg shadow-2xl mb-4 max-h-[500px] object-contain opacity-90 hover:opacity-100 transition-opacity",
                        alt: "Bradbury & Eggleston 1925 Paper"
                    }
                    p { class: "text-brand-taupe text-sm italic", "Bradbury S, Eggleston C. Postural Hypotension: A Report of Three Cases. Am Heart J. 1925." }
                }

                // RIGHT: Stats & Impact
                div { class: "flex flex-col justify-center space-y-8",
                     div { class: "bg-brand-green/10 rounded-2xl border border-brand-green/30 p-8 space-y-4",
                        h3 { class: "text-2xl font-bold text-brand-orange", "Prevalence" }
                        div {
                            div { class: "text-6xl font-bold text-brand-light", "16-30%" }
                            div { class: "text-xl text-brand-taupe mt-2", "of adults aged > 65 years" }
                        }
                    }

                    div { class: "bg-brand-green/10 rounded-2xl border border-brand-green/30 p-8 space-y-4",
                        h3 { class: "text-2xl font-bold text-brand-orange", "Economic Burden" }
                        div {
                            div { class: "text-6xl font-bold text-brand-light", "$86.8 M" }
                            div { class: "text-xl text-brand-taupe mt-2", "Annual Medicare Part D Spending (2023)" }
                            div { class: "text-brand-taupe/60 text-sm", "Midodrine, Fludrocortisone, Droxidopa" }
                        }
                    }
                }
            }
        }
    }
}
