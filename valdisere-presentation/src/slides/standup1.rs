use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
struct PatientData {
    mrn: String,
    pre: PhaseData,
    post: PhaseData,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
struct PhaseData {
    lying: Option<Vitals>,
    sitting: Option<Vitals>,
    standing: Option<Vitals>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
struct Vitals {
    sys: i32,
    dia: i32,
    hr: Option<i32>,
}

const DATA_JSON: &str = include_str!("../../assets/standup_data.json");

#[component]
pub fn Standup1() -> Element {
    let patients: Vec<PatientData> = serde_json::from_str(DATA_JSON).unwrap_or_default();

    // Calculate Averages
    let avg_stats = use_memo(move || {
        let mut pre_sys = [0, 0, 0]; // L, S, Std
        let mut pre_hr = [0, 0, 0];
        let mut pre_count = [0, 0, 0];
        let mut pre_hr_count = [0, 0, 0];

        let mut post_sys = [0, 0, 0];
        let mut post_hr = [0, 0, 0];
        let mut post_count = [0, 0, 0];
        let mut post_hr_count = [0, 0, 0];

        for p in &patients {
            // Pre
            if let Some(v) = &p.pre.lying {
                pre_sys[0] += v.sys;
                pre_count[0] += 1;
                if let Some(h) = v.hr {
                    pre_hr[0] += h;
                    pre_hr_count[0] += 1;
                }
            }
            if let Some(v) = &p.pre.sitting {
                pre_sys[1] += v.sys;
                pre_count[1] += 1;
                if let Some(h) = v.hr {
                    pre_hr[1] += h;
                    pre_hr_count[1] += 1;
                }
            }
            if let Some(v) = &p.pre.standing {
                pre_sys[2] += v.sys;
                pre_count[2] += 1;
                if let Some(h) = v.hr {
                    pre_hr[2] += h;
                    pre_hr_count[2] += 1;
                }
            }

            // Post
            if let Some(v) = &p.post.lying {
                post_sys[0] += v.sys;
                post_count[0] += 1;
                if let Some(h) = v.hr {
                    post_hr[0] += h;
                    post_hr_count[0] += 1;
                }
            }
            if let Some(v) = &p.post.sitting {
                post_sys[1] += v.sys;
                post_count[1] += 1;
                if let Some(h) = v.hr {
                    post_hr[1] += h;
                    post_hr_count[1] += 1;
                }
            }
            if let Some(v) = &p.post.standing {
                post_sys[2] += v.sys;
                post_count[2] += 1;
                if let Some(h) = v.hr {
                    post_hr[2] += h;
                    post_hr_count[2] += 1;
                }
            }
        }

        let calc_avg = |sum: i32, count: i32| if count > 0 { sum / count } else { 0 };

        (
            (
                [
                    calc_avg(pre_sys[0], pre_count[0]),
                    calc_avg(pre_sys[1], pre_count[1]),
                    calc_avg(pre_sys[2], pre_count[2]),
                ],
                [
                    calc_avg(pre_hr[0], pre_hr_count[0]),
                    calc_avg(pre_hr[1], pre_hr_count[1]),
                    calc_avg(pre_hr[2], pre_hr_count[2]),
                ],
            ),
            (
                [
                    calc_avg(post_sys[0], post_count[0]),
                    calc_avg(post_sys[1], post_count[1]),
                    calc_avg(post_sys[2], post_count[2]),
                ],
                [
                    calc_avg(post_hr[0], post_hr_count[0]),
                    calc_avg(post_hr[1], post_hr_count[1]),
                    calc_avg(post_hr[2], post_hr_count[2]),
                ],
            ),
        )
    });

    let ((pre_sys_avgs, pre_hr_avgs), (post_sys_avgs, post_hr_avgs)) = avg_stats();
    let max_sys = 180.0;
    let max_hr = 150.0;

    rsx! {
        div {
            class: "flex flex-col h-full w-full bg-zinc-950 text-white p-6 overflow-hidden",

            // Header
            div { class: "mb-6 animate-fade-in-down",
                h1 { class: "text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-cyan-400 to-blue-500 mb-2",
                    "Concepts & Results: Standup 1"
                }
                div { class: "h-1 w-32 bg-cyan-500 rounded-full" }
            }

            div { class: "flex flex-row gap-8 h-full",

                // Left Column: Concepts
                div { class: "w-1/3 space-y-4 animate-fade-in-left delay-300",
                    div { class: "p-5 bg-zinc-900/50 rounded-2xl border border-zinc-800 backdrop-blur-sm",
                        h3 { class: "text-xl font-semibold text-cyan-400 mb-2", "The Protocol" }
                        ul { class: "space-y-2 text-base text-slate-300",
                            li { class: "flex items-start", span { class: "mr-2 text-cyan-500", "•" } "Lying → Sitting → Standing (3m)" }
                            li { class: "flex items-start", span { class: "mr-2 text-cyan-500", "•" } "Metrics: BP & Heart Rate" }
                        }
                    }

                    div { class: "p-5 bg-zinc-900/50 rounded-2xl border border-zinc-800 backdrop-blur-sm",
                        h3 { class: "text-xl font-semibold text-cyan-400 mb-2", "Clinical Goals" }
                        ul { class: "space-y-2 text-base text-slate-300",
                            li { class: "flex items-start", span { class: "mr-2 text-cyan-500", "•" } "Assess venous return" }
                             li { class: "flex items-start", span { class: "mr-2 text-cyan-500", "•" } "Compare pre/post stenting stability" }
                        }
                    }
                }

                // Right Column: Data Visualization (2 Charts)
                div { class: "w-2/3 flex flex-col gap-4 animate-fade-in-up delay-500 overflow-y-auto pr-2",

                    // Systolic Chart
                    div {
                        h3 { class: "text-xl font-bold text-slate-200 mb-2", "Systolic BP (Mean)" }
                        div { class: "flex items-end justify-between h-48 bg-zinc-900/30 rounded-2xl p-4 border border-zinc-800 relative",
                             for i in 0..4 { div { class: "absolute w-full h-px bg-zinc-800/50", style: "bottom: {i * 25}%" } }
                            for (i, label) in ["Lying", "Sitting", "Standing"].iter().enumerate() {
                                div { class: "flex flex-col items-center gap-2 z-10 w-1/4 h-full justify-end group",
                                    div { class: "flex gap-2 items-end justify-center w-full h-full",
                                        div { class: "w-8 bg-rose-500/80 rounded-t-sm relative group/bar", style: "height: {(pre_sys_avgs[i] as f64 / max_sys) * 100.0}%",
                                            div { class: "absolute -top-6 left-1/2 -translate-x-1/2 opacity-0 group-hover/bar:opacity-100 text-rose-300 text-xs", "{pre_sys_avgs[i]}" } }
                                        div { class: "w-8 bg-emerald-500/80 rounded-t-sm relative group/bar", style: "height: {(post_sys_avgs[i] as f64 / max_sys) * 100.0}%",
                                             div { class: "absolute -top-6 left-1/2 -translate-x-1/2 opacity-0 group-hover/bar:opacity-100 text-emerald-300 text-xs", "{post_sys_avgs[i]}" } }
                                    }
                                    span { class: "text-sm text-slate-400", "{label}" }
                                }
                            }
                        }
                    }

                    // Heart Rate Chart
                    div {
                        h3 { class: "text-xl font-bold text-slate-200 mb-2", "Heart Rate (Mean)" }
                         div { class: "flex items-end justify-between h-48 bg-zinc-900/30 rounded-2xl p-4 border border-zinc-800 relative",
                             for i in 0..4 { div { class: "absolute w-full h-px bg-zinc-800/50", style: "bottom: {i * 25}%" } }
                            for (i, label) in ["Lying", "Sitting", "Standing"].iter().enumerate() {
                                div { class: "flex flex-col items-center gap-2 z-10 w-1/4 h-full justify-end group",
                                    div { class: "flex gap-2 items-end justify-center w-full h-full",
                                        div { class: "w-8 bg-purple-500/80 rounded-t-sm relative group/bar", style: "height: {(pre_hr_avgs[i] as f64 / max_hr) * 100.0}%",
                                            div { class: "absolute -top-6 left-1/2 -translate-x-1/2 opacity-0 group-hover/bar:opacity-100 text-purple-300 text-xs", "{pre_hr_avgs[i]}" } }
                                        div { class: "w-8 bg-cyan-500/80 rounded-t-sm relative group/bar", style: "height: {(post_hr_avgs[i] as f64 / max_hr) * 100.0}%",
                                             div { class: "absolute -top-6 left-1/2 -translate-x-1/2 opacity-0 group-hover/bar:opacity-100 text-cyan-300 text-xs", "{post_hr_avgs[i]}" } }
                                    }
                                    span { class: "text-sm text-slate-400", "{label}" }
                                }
                            }
                        }
                    }

                    // Legend
                    div { class: "flex justify-center gap-6 text-sm",
                        div { class: "flex items-center gap-2", div { class: "w-3 h-3 bg-rose-500 rounded" }, span { "Pre-Op BP" } }
                        div { class: "flex items-center gap-2", div { class: "w-3 h-3 bg-emerald-500 rounded" }, span { "Post-Op BP" } }
                        div { class: "flex items-center gap-2", div { class: "w-3 h-3 bg-purple-500 rounded" }, span { "Pre-Op HR" } }
                        div { class: "flex items-center gap-2", div { class: "w-3 h-3 bg-cyan-500 rounded" }, span { "Post-Op HR" } }
                    }
                }
            }
        }
    }
}
