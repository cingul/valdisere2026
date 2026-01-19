// app.js

// Global Chart Defaults for "Premium" Feel
Chart.defaults.font.family = '"Inter", sans-serif';
Chart.defaults.color = '#C5B7AB';
Chart.defaults.borderColor = 'rgba(255, 255, 255, 0.1)';

const THEME = {
    orange: '#F05708',
    green: '#283E28',
    greenLight: '#4ade80',
    taupe: '#C5B7AB',
    dark: '#0A0A0A',
    grid: 'rgba(255, 255, 255, 0.05)'
};

let chartInstances = {};

function destroyChart(id) {
    if (chartInstances[id]) {
        chartInstances[id].destroy();
        delete chartInstances[id];
    }
}

let loadedData = null;

// Register DataLabels globally if loaded
if (typeof ChartDataLabels !== 'undefined') {
    Chart.register(ChartDataLabels);
}

document.addEventListener('DOMContentLoaded', () => {
    loadData();
    initResultsObserver();
    initCounters();
    setupScrollAnimations();
    initOhDefinition();
});

function loadData() {
    try {
        loadedData = {
            bp: parseCSV(BP_CSV),
            meq: parseCSV(MEQ_CSV),
            cgi: parseCSV(CGI_CSV)
        };
    } catch (error) {
        console.error("Error loading data:", error);
    }
}

function initResultsObserver() {
    const section = document.getElementById('results');
    if (!section) return;

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                renderAllCharts();
            } else {
                // Destroy charts on exit to force re-animation on next entry
                Object.keys(chartInstances).forEach(destroyChart);

                // Reset Stat counters
                ['stat-pre-drop', 'stat-post-drop', 'stat-meq-rec'].forEach(id => {
                    const el = document.getElementById(id);
                    if (el) el.innerHTML = '---';
                });
            }
        });
    }, { threshold: 0.1 });

    observer.observe(section);
}

function renderAllCharts() {
    if (!loadedData) return;

    renderBPChart(loadedData.bp);
    renderMEQChart(loadedData.meq);
    renderCGIChart(loadedData.cgi);

    calculateStats(loadedData.bp, loadedData.meq);
}

function setupScrollAnimations() {
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.remove('opacity-0', 'translate-y-8');
                entry.target.classList.add('opacity-100', 'translate-y-0');
            } else {
                // Reset for replay
                entry.target.classList.add('opacity-0', 'translate-y-8');
                entry.target.classList.remove('opacity-100', 'translate-y-0');
            }
        });
    }, { threshold: 0.1 });

    document.querySelectorAll('.animate-on-scroll').forEach((el, index) => {
        // Intermediate duration: 1500ms
        el.classList.add('transition-all', 'duration-[1500ms]', 'ease-out', 'opacity-0', 'translate-y-8');
        // Stagger delay based on dataset or order
        const delay = el.dataset.delay || (index % 3) * 200;
        el.style.transitionDelay = `${delay}ms`;
        observer.observe(el);
    });
}
// --- Data (Embedded for Offline/Local File Support) ---

const BP_CSV = `Patient ID,Patient ,Mean Pre Orthostatic Systolic BP,Mean Post Orthostatic Systolic BP,Pre–Post Δ Orthostatic Systolic BP
E1000062,1,80,25.5,54.5
E1087852,2,10.33333333,7,3.333333333
E1227930,3,20,6.5,13.5
E136635,4,30.58823529,32.76470588,-2.176470588
E1493547,5,55.66666667,44.3,11.36666667
E1685922,6,38.95833333,24,14.95833333
E1912896,7,39.58823529,21.75,17.83823529
E1976264,8,28.33333333,-0.6666666667,29
E2027859,9,13,8,5
E2101192,10,59,35,24
E2318105,11,37.375,51.65384615,-14.27884615
E2464750,12,32.66666667,36,-3.333333333
E2470089,13,55.15384615,14,41.15384615
E2479375,14,40,9,31
E2489109,15,27,30.5,-3.5
E2548413,16,16,8,8
E2569638,17,17.5,15.66666667,1.833333333
E2604010,18,21,3,18
E276648,19,0.875,23.5,-22.625
E2831274,20,13.35714286,0.8,12.55714286
E2871516,21,49.90909091,20.2,29.70909091
E2932436,22,37.85714286,30.62857143,7.228571429
E311394,23,79,33,46
E3235226,24,35.86956522,23,12.86956522
E3910460,25,61.23529412,35.63636364,25.59893048
E3936196,26,49.625,39.25,10.375
E4194920,27,47.5,18.2,29.3
E4256828,28,86.875,75.55,11.325
E4559873,29,32.77777778,34.75,-1.972222222
E4663415,30,12,8.8,3.2
E466559,31,34.5,33.25,1.25
E4752058,32,31.14285714,14,17.14285714
E4791504,33,34,27,7
E4868119,34,21.81818182,30,-8.181818182
E4909045,35,62.83333333,47.5,15.33333333
E536330,36,30,42.66666667,-12.66666667
E938916,37,35,19,16`;

const MEQ_CSV = `PAT_MRN_ID,Patient,MEQ_Pre,MEQ_Post,Percent_Reduction
E136635,4,283.3,50,82.35086481
E1493547,5,166.7,133.3,20.0359928
E1685922,6,100,0,100
E1912896,7,283.3,50,82.35086481
E1976264,8,50,0,100
E2318105,11,16.7,0,100
E2464750,12,33.3,66.7,-100.3003003
E2470089,13,33.3,0,100
E2489109,15,33.3,0,100
E2569638,17,100,0,100
E2604010,18,83.3,0,100
E2831274,20,50,0,100
E2871516,21,100,50,50
E2932436,22,258.3,233.3,9.678668215
E3235226,24,300,0,100
E3910460,25,166.7,16.7,89.9820036
E3936196,26,122.2,33.3,72.74959083
E4194920,27,100,100,0
E4256828,28,166.7,108.3,35.0329934
E4868119,34,16.7,0,100
E4909045,35,233.3,0,100
E536330,36,0,33.3,-inf`;

const CGI_CSV = `PAT_MRN_ID,Patient,CGI-I Score
E1000062,1,4
E1087852,2,3
E1227930,3,1
E136635,4,4
E1493547,5,2
E1685922,6,2
E1912896,7,2
E1976264,8,3
E2027859,9,4
E2101192,10,3
E2318105,11,3
E2464750,12,2
E2470089,13,2
E2479375,14,2
E2489109,15,3
E2548413,16,3
E2569638,17,2
E2604010,18,1
E276648,19,4
E2831274,20,1
E2871516,21,3
E2932436,22,4
E311394,23,2
E3235226,24,1
E3910460,25,1
E3936196,26,2
E4194920,27,2
E4256828,28,2
E4559873,29,1
E4663415,30,1
E466559,31,1
E4752058,32,2
E4791504,33,5
E4868119,34,2
E4909045,35,2
E536330,36,4
E938916,37,2`;

const parseCSV = (csvText) => {
    return Papa.parse(csvText, { header: true, dynamicTyping: true, skipEmptyLines: true }).data;
};

// --- KPI Stats Calculation ---

function calculateStats(bpData, meqData) {
    // 1. Hemodynamics Stats (Average Drop)
    // Note: Column names are tricky with spaces. We use keys directly.
    // 'Mean Pre Orthostatic Systolic BP' -> Pre Drop
    // 'Mean Post Orthostatic Systolic BP' -> Post Drop

    // Helper to sum
    const sum = (arr) => arr.reduce((a, b) => a + b, 0);
    const avg = (arr) => arr.length ? (sum(arr) / arr.length).toFixed(1) : 0;

    const preDrops = bpData.map(d => d['Mean Pre Orthostatic Systolic BP'] || 0);
    const postDrops = bpData.map(d => d['Mean Post Orthostatic Systolic BP'] || 0);

    // Animate the numbers (Intermediate Duration: 2500ms)
    animateValue(document.getElementById('stat-pre-drop'), 0, avg(preDrops), 2500, ' mmHg');
    animateValue(document.getElementById('stat-post-drop'), 60, avg(postDrops), 2500, ' mmHg');

    // 2. MEQ Reduction Stats
    // Average of 'Percent_Reduction'. Filter out -inf/inf values.
    const reductions = meqData
        .map(d => d['Percent_Reduction'])
        .filter(v => Number.isFinite(v));

    // Duration Intermediate: 3000ms
    animateValue(document.getElementById('stat-meq-rec'), 0, avg(reductions), 3000, '%');
}


// --- Charts ---

function renderBPChart(originData) {
    destroyChart('chart-bp');

    if (!originData || originData.length === 0) return;

    // Robust Key Finding (Handles Dash/Space variants)
    const firstRow = originData[0];
    const keyPre = Object.keys(firstRow).find(k => k.includes('Mean Pre')) || 'Mean Pre Orthostatic Systolic BP';
    const keyPost = Object.keys(firstRow).find(k => k.includes('Mean Post')) || 'Mean Post Orthostatic Systolic BP';
    const keyDelta = Object.keys(firstRow).find(k => k.includes('Pre') && k.includes('Post') && k.includes('Orthostatic')) || 'Pre–Post Δ Orthostatic Systolic BP';
    const keyPatient = Object.keys(firstRow).find(k => k.trim().startsWith('Patient') && k !== 'Patient ID') || 'Patient ';

    // Clean Data
    const data = originData.filter(d => Number.isFinite(d[keyPost]));
    data.sort((a, b) => b[keyPre] - a[keyPre]);

    const labels = data.map(d => `Pt ${d[keyPatient]}`);
    const pre = data.map(d => d[keyPre]);
    const post = data.map(d => d[keyPost]);

    const ctx = document.getElementById('chart-bp').getContext('2d');
    chartInstances['chart-bp'] = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: labels,
            datasets: [
                {
                    label: 'Pre-Stent Drop',
                    data: pre,
                    backgroundColor: 'rgba(239, 68, 68, 0.7)', // Red
                    borderRadius: 4,
                },
                {
                    label: 'Post-Stent Drop',
                    data: post,
                    backgroundColor: 'rgba(74, 222, 128, 0.9)', // Green
                    borderRadius: 4,
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                datalabels: { display: false },
                title: { display: true, text: 'Orthostatic Systolic BP Drop (mmHg)', font: { size: 16 } },
                legend: { position: 'bottom' },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    backgroundColor: 'rgba(10, 10, 10, 0.9)',
                    titleColor: '#fff',
                    bodyColor: '#ccc',
                    borderColor: 'rgba(255,255,255,0.1)',
                    borderWidth: 1
                }
            },
            interaction: { mode: 'index', intersect: false },
            scales: {
                y: {
                    title: { display: true, text: 'Drop (mmHg)' },
                    grid: { color: THEME.grid }
                },
                x: { display: false } // Hide x labels if too many patients
            },
            animation: {
                y: {
                    duration: 2000,
                    easing: 'easeOutQuart',
                    delay: (c) => c.dataIndex * 200
                }
            }
        }
    });

    // Delta Chart (Improvement mmHg)
    destroyChart('chart-hr');
    const ctxDelta = document.getElementById('chart-hr').getContext('2d');
    const deltas = data.map(d => d[keyDelta]);

    chartInstances['chart-hr'] = new Chart(ctxDelta, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [{
                label: 'Systolic Improvement (mmHg)',
                data: deltas,
                borderColor: THEME.orange,
                backgroundColor: 'rgba(240, 87, 8, 0.1)',
                fill: true,
                tension: 0.4,
                pointBackgroundColor: '#fff',
                pointBorderColor: THEME.orange,
                pointRadius: 4
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            layout: { padding: { top: 10, bottom: 10 } },
            plugins: {
                datalabels: { display: false },
                title: { display: true, text: 'Individual Patient Improvement', font: { size: 16 } },
                legend: { display: false },
                tooltip: {
                    callbacks: {
                        label: (context) => ` Improvement: ${context.raw.toFixed(1)} mmHg`
                    }
                }
            },
            scales: {
                y: {
                    grid: { color: THEME.grid },
                    title: { display: true, text: 'Improvement (mmHg)', font: { size: 12, weight: 'bold' } }
                },
                x: { display: false }
            },
            animation: {
                y: { duration: 3000, from: 500 } // Intermediate 3000
            }
        }
    });
}

function renderMEQChart(data) {
    destroyChart('chart-meq');
    const ctx = document.getElementById('chart-meq').getContext('2d');

    // Sort by Pre MEQ
    data.sort((a, b) => b.MEQ_Pre - a.MEQ_Pre);

    const labels = data.map((d, i) => `Pt ${i + 1}`);
    const pre = data.map(d => d.MEQ_Pre);
    const post = data.map(d => d.MEQ_Post);

    chartInstances['chart-meq'] = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: labels,
            datasets: [
                {
                    label: 'Pre-Intervention Dose',
                    data: pre,
                    backgroundColor: 'rgba(239, 68, 68, 0.6)', // Red (Bad)
                    borderColor: 'rgba(239, 68, 68, 0.8)',
                    borderWidth: 1,
                    borderRadius: 4,
                    barPercentage: 0.8,
                    categoryPercentage: 0.9
                },
                {
                    label: 'Post-Intervention Dose',
                    data: post,
                    backgroundColor: 'rgba(74, 222, 128, 0.9)', // Green (Good)
                    borderColor: 'rgba(74, 222, 128, 1)',
                    borderWidth: 1,
                    borderRadius: 4,
                    barPercentage: 0.8,
                    categoryPercentage: 0.9
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                datalabels: { display: false },
                title: { display: true, text: 'Medication Burden (Midodrine Equivalents)', font: { size: 16 } },
                tooltip: {
                    callbacks: {
                        label: (ctx) => `${ctx.dataset.label}: ${ctx.raw} mg`
                    }
                },
                legend: { position: 'bottom' }
            },
            scales: {
                y: {
                    beginAtZero: true,
                    title: { display: true, text: 'Dose (mg)' },
                    grid: { color: THEME.grid }
                },
                x: { display: false }
            },
            animation: {
                duration: 2000,
                easing: 'easeOutQuart'
            }
        }
    });
}

function renderCGIChart(data) {
    destroyChart('chart-cgi');
    const ctx = document.getElementById('chart-cgi').getContext('2d');

    // Group By Score
    const counts = { 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0 };
    let total = 0;
    data.forEach(d => {
        let score = d['CGI-I Score'];
        if (counts[score] !== undefined) {
            counts[score]++;
            total++;
        }
    });

    // Included Score 5
    const chartData = [counts[1], counts[2], counts[3], counts[4], counts[5]];
    const chartLabels = ['Very Much Improved', 'Much Improved', 'Minimally Improved', 'No Change', 'Minimally Worse'];
    const chartColors = [THEME.greenLight, '#86efac', '#bbf7d0', '#52525b', '#ef4444'];

    chartInstances['chart-cgi'] = new Chart(ctx, {
        type: 'doughnut',
        data: {
            labels: chartLabels,
            datasets: [{
                data: chartData,
                backgroundColor: chartColors,
                borderWidth: 0,
                hoverOffset: 10
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            cutout: '55%',
            layout: { padding: 20 },
            plugins: {
                legend: {
                    position: 'right',
                    labels: { color: THEME.taupe, font: { size: 12 } }
                },
                title: { display: true, text: 'Patient Reported Outcomes', font: { size: 18 } },
                datalabels: {
                    display: true,
                    color: '#fff',
                    font: { weight: 'bold', size: 14 },
                    formatter: (value, ctx) => {
                        if (value === 0) return '';
                        return value; // Just count as requested
                    },
                    anchor: 'center',
                    align: 'center',
                    offset: 0
                }
            },
            animation: {
                animateScale: true,
                animateRotate: true,
                duration: 2500
            }
        }
    });
}


// --- Animation Helpers ---

function animateEconomicCounter(obj, start, end, duration) {
    if (!obj) return;
    let startTimestamp = null;
    const startColor = [197, 183, 171];
    const endColor = [239, 68, 68];

    const step = (timestamp) => {
        if (!startTimestamp) startTimestamp = timestamp;
        const progress = Math.min((timestamp - startTimestamp) / duration, 1);
        const ease = progress === 1 ? 1 : 1 - Math.pow(2, -10 * progress);

        // Update Value
        const current = Math.floor(start + (end - start) * ease);
        obj.innerHTML = '$' + current.toLocaleString('en-US');

        // Update Color
        const r = Math.round(startColor[0] + (endColor[0] - startColor[0]) * ease);
        const g = Math.round(startColor[1] + (endColor[1] - startColor[1]) * ease);
        const b = Math.round(startColor[2] + (endColor[2] - startColor[2]) * ease);
        obj.style.color = `rgb(${r}, ${g}, ${b})`;

        if (progress < 1) {
            window.requestAnimationFrame(step);
        }
    };
    window.requestAnimationFrame(step);
}

function animateValue(obj, start, end, duration, suffix = '') {
    if (!obj) return;
    let startTimestamp = null;
    const step = (timestamp) => {
        if (!startTimestamp) startTimestamp = timestamp;
        const progress = Math.min((timestamp - startTimestamp) / duration, 1);
        const ease = progress === 1 ? 1 : 1 - Math.pow(2, -10 * progress);

        const current = start + (end - start) * ease;
        obj.innerHTML = current.toFixed(1) + suffix;
        if (progress < 1) {
            window.requestAnimationFrame(step);
        }
    };
    window.requestAnimationFrame(step);
}

// --- Utils ---

window.switchTab = (tabId) => {
    document.querySelectorAll('.tab-content').forEach(tab => tab.classList.add('hidden'));
    document.getElementById(`view-${tabId}`).classList.remove('hidden');

    document.querySelectorAll('button[onclick^="switchTab"]').forEach(button => {
        button.classList.remove('bg-brand-orange', 'text-brand-dark', 'font-bold', 'border-brand-orange');
        button.classList.add('border-white/20', 'text-brand-taupe');
    });

    const activeBtn = document.getElementById(`tab-${tabId}`);
    activeBtn.classList.remove('border-white/20', 'text-brand-taupe');
    activeBtn.classList.add('bg-brand-orange', 'text-brand-dark', 'font-bold', 'border-brand-orange');

    // Force Re-render to replay animations
    setTimeout(() => {
        renderAllCharts();
    }, 50);
};


// --- Motivation Page Animations ---
function initCounters() {
    const c1 = document.getElementById('counter-total');
    const c2 = document.getElementById('counter-rx');

    if (!c1 || !c2) return;

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                animateEconomicCounter(c1, 0, 1000000000, 5000);
                animateEconomicCounter(c2, 0, 87000000, 5000);
            } else {
                c1.innerHTML = '$0';
                c2.innerHTML = '$0';
                c1.style.color = '#C5B7AB';
                c2.style.color = '#C5B7AB';
            }
        });
    }, { threshold: 0.2 });

    observer.observe(c1);
}

// --- OH Definition Animation ---
function initOhDefinition() {
    const section = document.getElementById('definition');
    const imgSupine = document.getElementById('img-supine');
    const imgSitting = document.getElementById('img-sitting');
    const imgStanding = document.getElementById('img-standing');
    const label = document.getElementById('posture-label');
    const bpValue = document.getElementById('bp-value');
    const annotation = document.getElementById('bp-annotation');

    if (!section) return;

    let timeouts = [];

    const resetState = () => {
        timeouts.forEach(t => clearTimeout(t));
        timeouts = [];

        imgSupine.classList.remove('opacity-0');
        imgSupine.classList.add('opacity-100');
        imgSitting.classList.remove('opacity-100');
        imgSitting.classList.add('opacity-0');
        imgStanding.classList.remove('opacity-100');
        imgStanding.classList.add('opacity-0');

        label.innerHTML = 'Lying Down';
        label.classList.remove('text-brand-orange', 'text-red-500', 'text-white');
        label.classList.add('text-brand-taupe');

        bpValue.innerHTML = '140';
        bpValue.style.color = 'white';
        annotation.classList.remove('opacity-100');
        annotation.classList.add('opacity-0');
    };

    const runSequence = () => {
        // Step 1: Sitting (T+1.5s)
        timeouts.push(setTimeout(() => {
            // Visuals
            imgSupine.classList.remove('opacity-100');
            imgSupine.classList.add('opacity-0');

            imgSitting.classList.remove('opacity-0');
            imgSitting.classList.add('opacity-100');

            // Label
            label.innerHTML = 'Sitting Up';
            label.classList.remove('text-brand-taupe');
            label.classList.add('text-white');

            // BP Drop (140 -> 130) (10pt drop)
            animateValue(bpValue, 140, 130, 2000);
        }, 1500));

        // Step 2: Standing (T+5s)
        timeouts.push(setTimeout(() => {
            // Visuals
            imgSitting.classList.remove('opacity-100');
            imgSitting.classList.add('opacity-0');

            imgStanding.classList.remove('opacity-0');
            imgStanding.classList.add('opacity-100');

            // Label
            label.innerHTML = 'Standing';
            label.classList.remove('text-white');
            label.classList.add('text-brand-orange');

            // BP Drop (130 -> 70) (Big drop)
            animateValue(bpValue, 130, 70, 2500);

            // Turn Red
            bpValue.style.color = '#ef4444';

            // Show Annotation
            setTimeout(() => {
                annotation.classList.remove('opacity-0');
                annotation.classList.add('opacity-100');
            }, 1000);

        }, 5000));
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                resetState();
                runSequence();
            } else {
                resetState();
            }
        });
    }, { threshold: 0.2 });

    observer.observe(section);
}

// ------------------------------------------------------------------
// Mortality Chart (ARIC Study)
// ------------------------------------------------------------------
function renderMortalityChart() {
    const ctx = document.getElementById('chart-mortality');
    if (!ctx) return;

    if (chartInstances['chart-mortality']) {
        chartInstances['chart-mortality'].destroy();
    }

    // Hazard Ratios
    const dataValues = [3.2, 2.4, 1.7];
    const labels = [
        ['Unadjusted', '(Raw Association)'],
        ['Demographic Adjusted', '(Age, Sex, Ethnicity)'],
        ['Fully Adjusted', '(CVD Factors, Comorbidities)']
    ];

    chartInstances['chart-mortality'] = new Chart(ctx.getContext('2d'), {
        type: 'bar',
        indexAxis: 'y', // Horizontal bars
        data: {
            labels: labels,
            datasets: [{
                label: 'Hazard Ratio (HR)',
                data: dataValues,
                backgroundColor: [
                    'rgba(240, 87, 8, 0.9)',   // Brand Orange
                    'rgba(240, 87, 8, 0.65)',  // Faded
                    'rgba(240, 87, 8, 0.4)'    // More Faded
                ],
                borderRadius: 6,
                barPercentage: 0.6,
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: { display: false },
                datalabels: {
                    color: '#fff',
                    anchor: 'end',
                    align: 'end',
                    font: { weight: 'bold', size: 14 },
                    formatter: (value) => value.toFixed(1) + 'x',
                    offset: 4
                },
                tooltip: {
                    backgroundColor: 'rgba(10, 10, 10, 0.9)',
                    titleColor: '#fff',
                    bodyColor: '#ccc',
                    callbacks: {
                        label: (ctx) => ` Hazard Ratio: ${ctx.raw}`
                    }
                },
                annotation: {
                    annotations: {
                        line1: {
                            type: 'line',
                            xMin: 1,
                            xMax: 1,
                            borderColor: '#fff',
                            borderWidth: 2,
                            borderDash: [6, 6],
                            label: {
                                display: true,
                                content: 'Baseline Risk (1.0)',
                                position: 'start',
                                color: 'rgba(255,255,255,0.7)',
                                font: { size: 10 },
                                yAdjust: -10
                            }
                        }
                    }
                }
            },
            scales: {
                x: {
                    min: 0,
                    max: 3.5,
                    grid: { color: 'rgba(255,255,255,0.05)' },
                    ticks: { color: '#999' },
                    title: { display: true, text: 'Hazard Ratio (Risk Factor)', color: '#666', font: { size: 10 } }
                },
                y: {
                    grid: { display: false },
                    ticks: {
                        color: '#eee',
                        font: { size: 12, family: 'sans-serif' },
                        crossAlign: 'far'
                    }
                }
            },
            animation: {
                duration: 2000,
                easing: 'easeOutQuart',
                delay: (context) => context.dataIndex * 300 // Staggered
            }
        }
    });
}

// ------------------------------------------------------------------
// Intervention Animation (Stenting Simulation)
// ------------------------------------------------------------------
function initInterventionAnimation() {
    const canvas = document.getElementById('canvas-intervention');
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
    let width, height;

    // Simulation State
    const state = {
        stented: false,
        expansion: 0, // 0 to 1
        stenosisLevel: 0.7, // 70% blocked initial
    };

    // UI Elements
    const btn = document.getElementById('btn-deploy-stent');
    const statusLabel = document.getElementById('stent-status');
    const flowLabel = document.getElementById('sim-flow-rate');


    // Resize Handler
    const resize = () => {
        const rect = canvas.getBoundingClientRect();
        // Fallback if not yet visible
        width = rect.width || canvas.clientWidth || 300;
        height = rect.height || canvas.clientHeight || 150;

        canvas.width = width * dpr;
        canvas.height = height * dpr;

        // Reset transform to avoid accumulation on multiple resizes
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.scale(dpr, dpr);

        // Re-spawn particles if they look invalid (first run)
        if (particles.length > 0 && isNaN(particles[0].x)) {
            initParticles();
        }
    };
    window.addEventListener('resize', resize);

    // Particles System
    let particles = [];
    const particleCount = 150;

    function initParticles() {
        particles = [];
        for (let i = 0; i < particleCount; i++) {
            particles.push({
                x: Math.random() * width,
                y: Math.random() * height,
                vx: Math.random() * 2 + 0.5,
                size: Math.random() * 2 + 1,
                color: 'rgba(239, 68, 68, 0.9)', // Bright Red (Tailwind red-500)
                offset: Math.random() * 100
            });
        }
    }

    // Initial resize 
    resize();
    initParticles();

    // Failsafe resize for layout shifts
    setTimeout(() => { resize(); }, 500);

    // Interaction
    if (btn) {
        btn.onclick = () => {
            state.stented = !state.stented;

            // Update UI Text
            if (state.stented) {
                btn.innerHTML = `<svg class="w-6 h-6 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path></svg> Reset Simulation`;
                btn.classList.remove('from-brand-orange', 'to-red-600');
                btn.classList.add('from-zinc-700', 'to-zinc-900', 'text-zinc-400');
            } else {
                btn.innerHTML = `<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg> Deploy Stent`;
                btn.classList.add('from-brand-orange', 'to-red-600');
                btn.classList.remove('from-zinc-700', 'to-zinc-900', 'text-zinc-400');
            }
        };
    }

    function animate() {
        // Clear with a lighter background (Zinc-800) to contrast with the black container
        ctx.fillStyle = '#27272a';
        ctx.fillRect(0, 0, width, height);

        const time = Date.now() / 1000;

        // Update Expansion State smooth transition
        if (state.stented && state.expansion < 1) {
            state.expansion += 0.015;
            if (state.expansion > 1) state.expansion = 1;
        } else if (!state.stented && state.expansion > 0) {
            state.expansion -= 0.02;
            if (state.expansion < 0) state.expansion = 0;
        }

        // Calculate current Pinch (Stenosis factor)
        // Expansion 0 => pinch is high (stenosisLevel). Expansion 1 => pinch is 0.
        const currentPinch = state.stenosisLevel * (1 - state.expansion);

        // Update UI Labels based on state
        if (state.expansion > 0.8) {
            statusLabel.innerHTML = `<span class="w-2 h-2 bg-brand-green rounded-full"></span> Flow Restored`;
            statusLabel.className = "flex items-center gap-2 text-brand-green font-bold text-sm uppercase tracking-widest";
            flowLabel.innerText = "NORMAL (LAMINAR)"; flowLabel.className = "text-brand-green font-bold";
        } else {
            statusLabel.innerHTML = `<span class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span> Stenosis Active`;
            statusLabel.className = "flex items-center gap-2 text-red-500 font-bold text-sm uppercase tracking-widest";
            flowLabel.innerText = "CRITICAL / TURBULENT"; flowLabel.className = "text-red-400 font-bold";
        }


        // Coordinate Helpers
        const midY = height / 2;
        const wallDist = height * 0.35; // Normal vessel radius

        // ---- Draw Walls ----
        ctx.beginPath();
        ctx.strokeStyle = '#4a1d1d'; // Dark blood red wall
        ctx.lineWidth = 12;
        ctx.lineCap = 'round';

        // Top Wall
        for (let x = 0; x <= width; x += 10) {
            // Function for wall shape: Base Y + Sine Wave + Pinch at center
            const centerDist = Math.abs(x - width / 2);
            const pinchEnvelope = Math.max(0, 1 - (centerDist / (width * 0.2))); // localized pinch

            // The pinch moves the wall INWARDS (positive Y for top wall)
            const y = (midY - wallDist) + (pinchEnvelope * currentPinch * wallDist * 0.9) + Math.sin(x / 50 + time) * 3;
            if (x === 0) ctx.moveTo(x, y);
            else ctx.lineTo(x, y);
        }
        ctx.stroke();

        // Bottom Wall
        ctx.beginPath();
        for (let x = 0; x <= width; x += 10) {
            const centerDist = Math.abs(x - width / 2);
            const pinchEnvelope = Math.max(0, 1 - (centerDist / (width * 0.2)));
            // Pinch moves wall INWARDS (negative Y offset from normal, or simpler: Y decreases towards mid)
            const y = (midY + wallDist) - (pinchEnvelope * currentPinch * wallDist * 0.9) + Math.sin(x / 50 + time) * 3;
            if (x === 0) ctx.moveTo(x, y);
            else ctx.lineTo(x, y);
        }
        ctx.stroke();

        // ---- Draw Stent (If expanding) ----
        if (state.expansion > 0.01) {
            ctx.save();
            ctx.globalAlpha = Math.min(1, state.expansion * 1.5); // Fade in

            const stentWidth = width * 0.5; // Stent covers center 50%
            const stentStartX = (width - stentWidth) / 2;
            const stentRadius = wallDist * 0.85 * (0.5 + 0.5 * state.expansion); // Grows

            ctx.strokeStyle = 'cyan';
            ctx.shadowColor = 'cyan';
            ctx.shadowBlur = 10;
            ctx.lineWidth = 1.5;

            // Draw Mesh Pattern
            ctx.beginPath();
            // Horizontal lines
            const topY = midY - stentRadius;
            const botY = midY + stentRadius;

            ctx.moveTo(stentStartX, topY);
            ctx.lineTo(stentStartX + stentWidth, topY);
            ctx.moveTo(stentStartX, botY);
            ctx.lineTo(stentStartX + stentWidth, botY);

            // Cross hatch
            for (let i = 0; i <= stentWidth; i += 15) {
                // Diagonals / Diamonds
                ctx.moveTo(stentStartX + i, topY);
                ctx.lineTo(stentStartX + i + 10, botY);

                ctx.moveTo(stentStartX + i + 10, topY);
                ctx.lineTo(stentStartX + i, botY);
            }
            ctx.stroke();
            ctx.restore();
        }

        // ---- Draw Particles (Blood Flow) ----
        particles.forEach(p => {
            // Logic: Update Position

            // Speed calculation
            let speed = p.vx;

            // If Stenosis is active, slow down near center
            const distToCenterX = Math.abs(p.x - width / 2);
            if (currentPinch > 0.1 && distToCenterX < width * 0.2) {
                speed *= 0.3; // Traffic jam
                p.y += (Math.random() - 0.5) * 2; // Turbulence
            } else if (state.stented && state.expansion > 0.8) {
                speed *= 2.5; // Accelerated Laminar Flow!
            }

            p.x += speed;

            // Wrap around
            if (p.x > width) p.x = -10;

            // Constrain Y (Keep inside vessel)
            // Calculate local vessel bounds at particle X
            const pinchEnv = Math.max(0, 1 - (Math.abs(p.x - width / 2) / (width * 0.2)));
            const topLimit = (midY - wallDist) + (pinchEnv * currentPinch * wallDist * 0.8) + 15;
            const botLimit = (midY + wallDist) - (pinchEnv * currentPinch * wallDist * 0.8) - 15;

            if (p.y < topLimit) p.y = topLimit + Math.random();
            if (p.y > botLimit) p.y = botLimit - Math.random();

            // Draw
            ctx.beginPath();
            // Color shift: Darker when slow/congested, Brighter when fast
            if (speed < 1) ctx.fillStyle = 'rgba(100, 20, 20, 0.8)';
            else ctx.fillStyle = 'rgba(255, 60, 60, 0.9)';

            ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
            ctx.fill();
        });

        requestAnimationFrame(animate);
    }

    animate();
}

document.addEventListener('DOMContentLoaded', () => {
    // Existing results charts observer
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                if (window.globalData) {
                    renderAllCharts(window.globalData);
                }
                observer.unobserve(entry.target);
            }
        });
    }, { threshold: 0.1 }); // Lower threshold for earlier loading

    const resultsSection = document.getElementById('results');
    if (resultsSection) observer.observe(resultsSection);

    // Mortality Chart Observer
    const mortalityObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                renderMortalityChart();
                mortalityObserver.unobserve(entry.target);
            }
        });
    }, { threshold: 0.2 });

    const motivationSection = document.getElementById('motivation');
    if (motivationSection) mortalityObserver.observe(motivationSection);

    // Start Intervention Animation
    initInterventionAnimation();
});
