use dioxus::prelude::*;
use std::collections::HashMap;

const CSV_BP: &str = include_str!("../../assets/standup_results.csv");
const CSV_MEQ: &str = include_str!("../../assets/standup_results_meq.csv");
const CSV_CGI: &str = include_str!("../../assets/standup_results_cgi.csv");

#[derive(Debug, Clone, PartialEq, Copy)]
enum Tab {
    Hemodynamics,
    Medication,
    Clinical,
}

// --- Hemodynamics Types ---
#[derive(Debug, Clone, PartialEq)]
enum Position {
    Lying,
    Sitting,
    Standing,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
enum Phase {
    Pre,
    Post,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
struct BpRecord {
    sys: Option<i32>,
    hr: Option<i32>,
    pos: Position,
    phase: Phase,
}

#[derive(Default, Clone, Copy, PartialEq)]
struct BpStats {
    sys_sum: i32,
    sys_count: i32,
    hr_sum: i32,
    hr_count: i32,
}
impl BpStats {
    fn avg_sys(&self) -> i32 {
        if self.sys_count > 0 {
            self.sys_sum / self.sys_count
        } else {
            0
        }
    }
    fn avg_hr(&self) -> i32 {
        if self.hr_count > 0 {
            self.hr_sum / self.hr_count
        } else {
            0
        }
    }
}

// --- Medication Types ---
#[derive(Debug, Clone, PartialEq)]
struct MeqRecord {
    pre: f32,
    post: f32,
}

// --- CGI Types ---
#[derive(Debug, Clone, PartialEq)]
struct CgiRecord {
    score: i32,
}

// --- Parsers ---
fn parse_int(s: &str) -> Option<i32> {
    s.trim().parse::<i32>().ok()
}
fn parse_float(s: &str) -> Option<f32> {
    s.trim().parse::<f32>().ok()
}

fn parse_bp_data(content: &str) -> Vec<BpRecord> {
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() < 7 {
                return None;
            }

            let sys = parse_int(cols[2]);
            let hr = parse_int(cols[4]);

            let pos = match cols[5].trim().to_lowercase().as_str() {
                "lying" => Position::Lying,
                "sitting" => Position::Sitting,
                "standing" => Position::Standing,
                _ => Position::Unknown,
            };

            let phase = if cols[6].to_lowercase().contains("pre") {
                Phase::Pre
            } else if cols[6].to_lowercase().contains("post") {
                Phase::Post
            } else {
                Phase::Unknown
            };

            if pos == Position::Unknown || phase == Phase::Unknown {
                return None;
            }
            Some(BpRecord {
                sys,
                hr,
                pos,
                phase,
            })
        })
        .collect()
}

fn parse_meq_data(content: &str) -> Vec<MeqRecord> {
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() < 4 {
                return None;
            }
            let pre = parse_float(cols[2])?;
            let post = parse_float(cols[3])?;
            Some(MeqRecord { pre, post })
        })
        .collect()
}

fn parse_cgi_data(content: &str) -> Vec<CgiRecord> {
    content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() < 3 {
                return None;
            }
            let score = parse_int(cols[2])?;
            Some(CgiRecord { score })
        })
        .collect()
}

#[component]
pub fn Results() -> Element {
    let mut active_tab = use_signal(|| Tab::Hemodynamics);

    // -- Data Memos --
    let bp_data = use_memo(|| parse_bp_data(CSV_BP));
    let meq_data = use_memo(|| parse_meq_data(CSV_MEQ));
    let cgi_data = use_memo(|| parse_cgi_data(CSV_CGI));

    // -- Hemodynamic Aggregation --
    let bp_stats = use_memo(move || {
        let mut pre = [BpStats::default(); 3];
        let mut post = [BpStats::default(); 3];
        for r in bp_data.read().iter() {
            let idx = match r.pos {
                Position::Lying => 0,
                Position::Sitting => 1,
                Position::Standing => 2,
                _ => continue,
            };
            let target = match r.phase {
                Phase::Pre => &mut pre[idx],
                Phase::Post => &mut post[idx],
                _ => continue,
            };
            if let Some(s) = r.sys {
                target.sys_sum += s;
                target.sys_count += 1;
            }
            if let Some(h) = r.hr {
                target.hr_sum += h;
                target.hr_count += 1;
            }
        }
        (pre, post)
    });
    let (pre_bp, post_bp) = *bp_stats.read();
    let pre_drop = pre_bp[0].avg_sys() - pre_bp[2].avg_sys();
    let post_drop = post_bp[0].avg_sys() - post_bp[2].avg_sys();

    // -- MEQ Aggregation --
    let meq_stats = use_memo(move || {
        let recs = meq_data.read();
        let total_pre: f32 = recs.iter().map(|r| r.pre).sum();
        let total_post: f32 = recs.iter().map(|r| r.post).sum();
        let count = recs.len() as f32;
        if count == 0.0 {
            return (0.0, 0.0);
        }
        (total_pre / count, total_post / count)
    });
    let (avg_meq_pre, avg_meq_post) = *meq_stats.read();
    let meq_reduction = if avg_meq_pre > 0.0 {
        ((avg_meq_pre - avg_meq_post) / avg_meq_pre) * 100.0
    } else {
        0.0
    };

    // -- CGI Aggregation --
    let cgi_dist = use_memo(move || {
        let mut counts = [0; 8]; // 1-7 (score 0 unused)
        for r in cgi_data.read().iter() {
            if r.score >= 1 && r.score <= 7 {
                counts[r.score as usize] += 1;
            }
        }
        counts
    });
    let cgi_counts = *cgi_dist.read();

    rsx! {
        div { class: "flex flex-col min-h-full w-full bg-brand-dark text-brand-light p-8",

            // Header with Tabs
            div { class: "z-10 mb-6 flex justify-between items-end animate-fade-in-down",
                div {
                     h1 { class: "text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-brand-orange to-orange-400 mb-4",
                        "Study Results"
                    }
                    div { class: "flex gap-2",
                        TabButton { active: active_tab() == Tab::Hemodynamics, label: "Hemodynamics", onclick: move |_| active_tab.set(Tab::Hemodynamics) }
                        TabButton { active: active_tab() == Tab::Medication, label: "Medication", onclick: move |_| active_tab.set(Tab::Medication) }
                        TabButton { active: active_tab() == Tab::Clinical, label: "Clinical Outcomes", onclick: move |_| active_tab.set(Tab::Clinical) }
                    }
                }
                div { class: "text-right pb-2",
                    div { class: "text-brand-taupe text-sm", "STANDUP Cohort N=37" }
                }
            }

            // Tab Content
            div { class: "flex-1 overflow-visible animate-fade-in-up",
                match active_tab() {
                    Tab::Hemodynamics => rsx! {
                        div { class: "grid grid-cols-12 gap-8 h-full pb-4",
                            // Key Metrics
                             div { class: "col-span-3 flex flex-col gap-6",
                                 div { class: "p-6 bg-brand-green/10 rounded-2xl border border-brand-green/30 space-y-4",
                                    h3 { class: "text-lg font-bold text-brand-orange uppercase", "Systolic Drop" }
                                    div { class: "text-sm text-brand-taupe", "Supine to Standing" }
                                    div { class: "flex flex-col gap-1",
                                        span { class: "text-xs text-brand-taupe uppercase", "Pre-Intervention" }
                                        span { class: "text-4xl font-mono font-bold text-red-400", "-{pre_drop} mmHg" }
                                    }
                                    div { class: "w-full h-px bg-brand-green/20" }
                                    div { class: "flex flex-col gap-1",
                                        span { class: "text-xs text-brand-taupe uppercase", "Post-Intervention" }
                                        span { class: "text-4xl font-mono font-bold text-brand-green", "-{post_drop} mmHg" }
                                    }
                                }
                                div { class: "p-6 bg-brand-green/5 rounded-2xl border border-brand-green/10",
                                     p { class: "text-sm text-brand-taupe italic", "Summary: Significant hemodynamic stabilization achieved." }
                                }
                            }
                            // Graphs
                            div { class: "col-span-9 grid grid-cols-2 gap-6",
                                ChartBox { title: "Systolic BP Profile", color: "orange",
                                    labels: vec!["Lying", "Sitting", "Standing"],
                                    data_a: vec![pre_bp[0].avg_sys(), pre_bp[1].avg_sys(), pre_bp[2].avg_sys()],
                                    data_b: vec![post_bp[0].avg_sys(), post_bp[1].avg_sys(), post_bp[2].avg_sys()],
                                    legend: ("Pre-Op", "Post-Op"),
                                    scale: 200.0
                                }
                                ChartBox { title: "Heart Rate Profile", color: "blue",
                                    labels: vec!["Lying", "Sitting", "Standing"],
                                    data_a: vec![pre_bp[0].avg_hr(), pre_bp[1].avg_hr(), pre_bp[2].avg_hr()],
                                    data_b: vec![post_bp[0].avg_hr(), post_bp[1].avg_hr(), post_bp[2].avg_hr()],
                                    legend: ("Pre-Op", "Post-Op"),
                                    scale: 150.0
                                }
                            }
                        }
                    },
                    Tab::Medication => rsx! {
                         div { class: "grid grid-cols-2 gap-12 h-full items-center px-12",
                            div { class: "flex flex-col gap-8",
                                div { class: "text-6xl font-bold text-brand-light",
                                    "{meq_reduction:.1}%"
                                    span { class: "text-3xl text-brand-green block mt-2", "Reduction in Medication" }
                                }
                                p { class: "text-xl text-brand-taupe leading-relaxed",
                                    "Midodrine Equivalent Dose (MEQ) significantly decreased post-intervention, indicating reduced pharmaceutical dependence."
                                }
                            }
                            div {
                                ChartBox { title: "Avg. Daily MEQ Dose", color: "green",
                                    labels: vec!["Pre-Intervention", "Post-Intervention"],
                                    data_a: vec![avg_meq_pre as i32, 0],
                                    data_b: vec![0, avg_meq_post as i32],
                                    legend: ("Pre (mg)", "Post (mg)"),
                                    scale: (avg_meq_pre * 1.2) as f64
                                }
                            }
                        }
                    },
                    Tab::Clinical => rsx! {
                         div { class: "flex flex-col gap-8 h-full px-8",
                            div { class: "text-center",
                                h2 { class: "text-2xl text-brand-light font-bold mb-2", "Clinical Global Impression - Improvement (CGI-I)" }
                                p { class: "text-brand-taupe", "1 = Very Much Improved, 2 = Much Improved" }
                            }
                            div { class: "flex-1 flex items-end justify-center gap-4 bg-zinc-900/50 rounded-2xl p-8 border border-zinc-700/50",
                                {(1..=7).filter(|s| cgi_counts[*s] > 0).map(|score| {
                                    let count = cgi_counts[score as usize];
                                    let max = *cgi_counts.iter().max().unwrap_or(&1) as f64;
                                    let h_pct = (count as f64 / max) * 100.0;
                                    let label = match score { 1 => "Very Much Improved", 2 => "Much Improved", 3 => "Minimally Improved", 4 => "No Change", _ => "" };
                                    rsx! {
                                        div { class: "flex flex-col items-center gap-2 w-32 group",
                                            div { class: "relative w-full bg-brand-green/20 rounded-t-lg transition-all group-hover:bg-brand-green/40 min-h-[10px]", style: "height: {h_pct}%",
                                                div { class: "absolute -top-8 left-1/2 -translate-x-1/2 font-bold text-2xl text-brand-light", "{count}" }
                                            }
                                            div { class: "text-center text-sm font-bold text-brand-orange", "{score}" }
                                            div { class: "text-center text-xs text-brand-taupe", "{label}" }
                                        }
                                    }
                                })}
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn TabButton(active: bool, label: String, onclick: EventHandler<MouseEvent>) -> Element {
    let base_class =
        "px-6 py-2 rounded-full font-bold transition-all text-sm cursor-pointer border";
    let active_class =
        "bg-brand-orange text-brand-dark border-brand-orange shadow-[0_0_15px_rgba(240,87,8,0.4)]";
    let inactive_class = "bg-transparent text-brand-taupe border-white/10 hover:border-brand-orange/50 hover:text-brand-light";

    let state_class = if active { active_class } else { inactive_class };

    rsx! {
        button {
            class: "{base_class} {state_class}",
            onclick: onclick,
            "{label}"
        }
    }
}

#[component]
fn ChartBox(
    title: String,
    color: String,
    labels: Vec<&'static str>,
    data_a: Vec<i32>,
    data_b: Vec<i32>,
    legend: (&'static str, &'static str),
    scale: f64,
) -> Element {
    let color_a = if color == "orange" {
        "bg-gradient-to-t from-red-900/80 to-red-500"
    } else {
        "bg-gradient-to-t from-zinc-700 to-zinc-500"
    };
    let color_b = if color == "orange" {
        "bg-gradient-to-t from-brand-green/80 to-emerald-400"
    } else if color == "green" {
        "bg-gradient-to-t from-brand-green/80 to-emerald-400"
    } else {
        "bg-gradient-to-t from-cyan-900/80 to-cyan-400"
    };

    rsx! {
        div { class: "bg-zinc-900/50 rounded-2xl p-6 border border-zinc-700/50 flex flex-col",
            h3 { class: "text-xl font-bold text-brand-light mb-6", "{title}" }
            div { class: "flex-1 relative flex items-end justify-around px-4 pb-8 min-h-[250px]",
                {labels.iter().enumerate().map(|(i, label)| rsx! {
                    div { class: "relative group flex gap-2 items-end h-full w-full justify-center mx-2",
                        if data_a[i] > 0 {
                            div { class: "w-12 {color_a} rounded-t-sm relative hover:brightness-110 transition-all", style: "height: {(data_a[i] as f64 / scale) * 100.0}%",
                                div { class: "absolute -top-6 left-1/2 -translate-x-1/2 text-xs font-bold text-zinc-400 opacity-0 group-hover:opacity-100", "{data_a[i]}" }
                            }
                        }
                        if data_b[i] > 0 {
                            div { class: "w-12 {color_b} rounded-t-sm relative hover:brightness-110 transition-all", style: "height: {(data_b[i] as f64 / scale) * 100.0}%",
                                div { class: "absolute -top-6 left-1/2 -translate-x-1/2 text-xs font-bold text-brand-green opacity-0 group-hover:opacity-100", "{data_b[i]}" }
                            }
                        }
                        div { class: "absolute -bottom-8 text-sm text-zinc-400 font-medium", "{label}" }
                    }
                })}
            }
            div { class: "mt-4 flex justify-center gap-6 text-sm",
                div { class: "flex items-center gap-2", div { class: "w-3 h-3 {color_a} rounded-sm" }, span { class: "text-zinc-400", "{legend.0}" } }
                div { class: "flex items-center gap-2", div { class: "w-3 h-3 {color_b} rounded-sm" }, span { class: "text-zinc-400", "{legend.1}" } }
             }
        }
    }
}
