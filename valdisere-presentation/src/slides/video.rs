use dioxus::prelude::*;

#[component]
pub fn CaseVideo() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-black text-brand-light p-12 relative overflow-hidden",

             div { class: "z-10 mb-8 animate-fade-in-down",
                h1 { class: "text-4xl font-bold text-brand-taupe",
                    "Case Study"
                }
                div { class: "h-1 w-24 bg-brand-orange rounded-full mt-2" }
            }

            // Video Container
            div { class: "flex-grow flex items-center justify-center z-10 animate-fade-in-up",
                div { class: "relative w-full max-w-5xl aspect-video bg-zinc-900 rounded-2xl border border-brand-green/30 shadow-2xl overflow-hidden group",

                    // Video Element
                    video {
                        class: "w-full h-full object-cover",
                        controls: true,
                        autoplay: false,
                        src: "assets/case_video.mp4",
                        "Your browser does not support the video tag."
                    }

                    // Fallback
                    div { class: "absolute inset-0 flex items-center justify-center pointer-events-none group-hover:opacity-0 transition-opacity bg-black/50 backdrop-blur-sm",
                        div { class: "text-center",
                            div { class: "text-6xl text-brand-orange mb-4", "▶" }
                            p { class: "text-brand-taupe font-medium", "Patient 102 - 6 Month Follow-up" }
                        }
                    }
                }
            }
        }
    }
}
