use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Choice {
    label: &'static str,
    co2: f64,
    fact: &'static str,
}

#[derive(Clone)]
struct Stage {
    title: &'static str,
    description: &'static str,
    icon: &'static str,
    choices: Vec<Choice>,
}

struct GameState {
    current_stage: usize,
    total_co2: f64,
    showing_result: bool,
    last_choice_co2: f64,
    last_choice_fact: String,
    game_over: bool,
    stages: Vec<Stage>,
}

impl GameState {
    fn new() -> Self {
        let stages = vec![
            Stage {
                title: "Raw Material",
                description: "Every garment begins with a fiber. The choice of raw material sets the foundation for the entire carbon footprint. What fiber will your garment be made from?",
                icon: "1",
                choices: vec![
                    Choice {
                        label: "Organic Cotton",
                        co2: 2.5,
                        fact: "Organic cotton uses 91% less water and no synthetic pesticides. It produces about 46% less CO2 than conventional cotton farming.",
                    },
                    Choice {
                        label: "Conventional Cotton",
                        co2: 5.0,
                        fact: "Conventional cotton uses heavy pesticides and fertilizers. It takes about 2,700 liters of water to make one cotton t-shirt.",
                    },
                    Choice {
                        label: "Polyester",
                        co2: 7.5,
                        fact: "Polyester is made from petroleum and takes 200+ years to decompose. Production emits 3x more CO2 than cotton.",
                    },
                ],
            },
            Stage {
                title: "Manufacturing",
                description: "Your fiber heads to the factory to be spun, woven, and cut into a garment. The type of factory matters enormously for both emissions and ethics.",
                icon: "2",
                choices: vec![
                    Choice {
                        label: "Renewable Energy Factory",
                        co2: 1.5,
                        fact: "Factories powered by solar and wind can reduce manufacturing emissions by up to 80%. Fair wages and safe conditions are standard.",
                    },
                    Choice {
                        label: "Standard Factory",
                        co2: 4.0,
                        fact: "Most garment factories run on coal or natural gas. The fashion industry accounts for 10% of global carbon emissions.",
                    },
                    Choice {
                        label: "Unregulated Sweatshop",
                        co2: 6.5,
                        fact: "Unregulated factories often use the dirtiest energy sources, dump waste illegally, and exploit workers with unsafe conditions and poverty wages.",
                    },
                ],
            },
            Stage {
                title: "Dyeing & Treatment",
                description: "Time to add color and finish to the fabric. Textile dyeing is one of the most polluting steps in fashion, responsible for 20% of global water pollution.",
                icon: "3",
                choices: vec![
                    Choice {
                        label: "Natural Dyes",
                        co2: 1.0,
                        fact: "Plant-based dyes from indigo, turmeric, and madder root are biodegradable and non-toxic. They require more material but produce zero synthetic waste.",
                    },
                    Choice {
                        label: "Low-Impact Synthetic",
                        co2: 2.5,
                        fact: "Low-impact dyes have a higher absorption rate (95%+), meaning less rinse water and chemical waste compared to conventional dyes.",
                    },
                    Choice {
                        label: "Conventional Chemical Dyes",
                        co2: 5.0,
                        fact: "Conventional dyeing uses 8,000+ synthetic chemicals. Wastewater is often dumped into rivers, poisoning ecosystems and communities downstream.",
                    },
                ],
            },
            Stage {
                title: "Shipping",
                description: "Your garment is ready to travel to stores worldwide. How it gets there makes a huge difference in its carbon footprint.",
                icon: "4",
                choices: vec![
                    Choice {
                        label: "Local Production / Train",
                        co2: 0.5,
                        fact: "Locally made garments travel minimal distances. Rail freight produces 75% fewer emissions than trucks and 95% less than air freight.",
                    },
                    Choice {
                        label: "Container Ship",
                        co2: 3.0,
                        fact: "90% of world trade travels by sea. A single large container ship can emit as much pollution as 50 million cars in a year.",
                    },
                    Choice {
                        label: "Air Freight",
                        co2: 8.0,
                        fact: "Air freight produces 50x more CO2 per km than shipping by sea. Fast fashion brands increasingly use air freight to keep up with trends.",
                    },
                ],
            },
            Stage {
                title: "Consumer Use",
                description: "The garment is yours! How you care for it over its lifetime has a surprising impact. About 25% of a garment's total carbon footprint comes from washing and drying.",
                icon: "5",
                choices: vec![
                    Choice {
                        label: "Cold Wash / Line Dry",
                        co2: 1.0,
                        fact: "Washing at 30C instead of 60C uses 57% less energy. Line drying saves about 2.4 kg of CO2 per load compared to machine drying.",
                    },
                    Choice {
                        label: "Warm Wash / Tumble Dry",
                        co2: 3.5,
                        fact: "A tumble dryer is one of the most energy-hungry home appliances. The average household does 400 loads per year.",
                    },
                    Choice {
                        label: "Frequent Hot Wash / Dry Clean",
                        co2: 6.0,
                        fact: "Hot washing degrades fabrics faster, shortening garment life. Dry cleaning uses perchloroethylene, a toxic chemical linked to health problems.",
                    },
                ],
            },
            Stage {
                title: "End of Life",
                description: "Every garment eventually reaches the end of its wearable life. What happens next determines whether it becomes waste or gets a second chance.",
                icon: "6",
                choices: vec![
                    Choice {
                        label: "Repair & Rewear",
                        co2: 0.5,
                        fact: "Extending a garment's life by just 9 months reduces its carbon, water, and waste footprint by 20-30%. Simple repairs can double a garment's lifespan.",
                    },
                    Choice {
                        label: "Donate / Thrift",
                        co2: 1.5,
                        fact: "Donating keeps clothes in circulation. However, only 10-20% of donated clothes are resold locally; much is exported or ends up in landfill anyway.",
                    },
                    Choice {
                        label: "Recycle",
                        co2: 3.0,
                        fact: "Less than 1% of clothing is recycled into new garments. Most textile recycling downcycles fabric into insulation, rags, or mattress stuffing.",
                    },
                    Choice {
                        label: "Landfill",
                        co2: 6.0,
                        fact: "92 million tons of textiles end up in landfills each year. Synthetic fabrics take 200+ years to decompose, releasing methane and toxic chemicals.",
                    },
                ],
            },
        ];

        GameState {
            current_stage: 0,
            total_co2: 0.0,
            showing_result: false,
            last_choice_co2: 0.0,
            last_choice_fact: String::new(),
            game_over: false,
            stages,
        }
    }
}

fn get_grade(co2: f64) -> (&'static str, &'static str) {
    if co2 <= 10.0 {
        ("A", "#22C55E")
    } else if co2 <= 16.0 {
        ("B", "#84CC16")
    } else if co2 <= 22.0 {
        ("C", "#EAB308")
    } else if co2 <= 30.0 {
        ("D", "#F97316")
    } else {
        ("F", "#EF4444")
    }
}

fn render(state: &GameState, _document: &Document, container: &Element) {
    let industry_avg: f64 = 33.4;

    if state.game_over {
        let (grade, grade_color) = get_grade(state.total_co2);
        let comparison = if state.total_co2 < industry_avg {
            format!("{:.1} kg less than", industry_avg - state.total_co2)
        } else {
            format!("{:.1} kg more than", state.total_co2 - industry_avg)
        };

        let max_co2: f64 = 40.0;
        let bar_pct = (state.total_co2 / max_co2 * 100.0).min(100.0);
        let avg_pct = (industry_avg / max_co2 * 100.0).min(100.0);

        let html = format!(r#"
            <div class="ct-header">
                <div class="ct-stage-label">Journey Complete</div>
                <div class="ct-co2-display">{:.1} <span class="ct-co2-unit">kg CO2</span></div>
            </div>
            <div class="ct-content ct-results">
                <div class="ct-grade-circle" style="border-color: {};">{}</div>
                <div class="ct-result-text">
                    Your garment produced <strong>{:.1} kg CO2</strong> across its entire lifecycle.
                </div>
                <div class="ct-comparison-bar-container">
                    <div class="ct-comparison-label">Your garment vs. industry average ({:.1} kg)</div>
                    <div class="ct-comparison-track">
                        <div class="ct-comparison-fill ct-your-fill" style="width: {:.1}%; background: {};"></div>
                        <div class="ct-comparison-marker" style="left: {:.1}%;" title="Industry avg: {:.1} kg">
                            <div class="ct-marker-line"></div>
                            <div class="ct-marker-label">Avg</div>
                        </div>
                    </div>
                </div>
                <div class="ct-result-text ct-comparison-text">
                    That's <strong>{}</strong> the industry average of {:.1} kg CO2.
                </div>
                <button class="ct-btn ct-btn-primary" id="ct-play-again">Play Again</button>
            </div>
        "#, state.total_co2, grade_color, grade, state.total_co2,
            industry_avg, bar_pct, grade_color, avg_pct, industry_avg,
            comparison, industry_avg);
        container.set_inner_html(&html);
        return;
    }

    if state.showing_result {
        let stage = &state.stages[state.current_stage];
        let max_co2: f64 = 40.0;
        let bar_pct = (state.total_co2 / max_co2 * 100.0).min(100.0);

        let html = format!(r#"
            <div class="ct-header">
                <div class="ct-stage-info">
                    <span class="ct-stage-num">Stage {} of 6</span>
                    <span class="ct-stage-title">{}</span>
                </div>
                <div class="ct-co2-display">{:.1} <span class="ct-co2-unit">kg CO2</span></div>
                <div class="ct-progress-bar"><div class="ct-progress-fill" style="width: {:.1}%;"></div></div>
            </div>
            <div class="ct-content ct-result-view">
                <div class="ct-impact-badge">+{:.1} kg CO2</div>
                <div class="ct-fact-box">
                    <p>{}</p>
                </div>
                <button class="ct-btn ct-btn-primary" id="ct-continue">Continue</button>
            </div>
        "#, state.current_stage + 1, stage.title,
            state.total_co2, bar_pct,
            state.last_choice_co2, state.last_choice_fact);
        container.set_inner_html(&html);
        return;
    }

    let stage = &state.stages[state.current_stage];
    let max_co2: f64 = 40.0;
    let bar_pct = (state.total_co2 / max_co2 * 100.0).min(100.0);

    let mut choices_html = String::new();
    for (i, choice) in stage.choices.iter().enumerate() {
        let eco_level = if choice.co2 <= 1.5 {
            ("Low Impact", "ct-eco-low")
        } else if choice.co2 <= 4.0 {
            ("Medium Impact", "ct-eco-med")
        } else {
            ("High Impact", "ct-eco-high")
        };
        choices_html.push_str(&format!(
            r#"<button class="ct-choice-btn" data-choice="{}">
                <span class="ct-choice-label">{}</span>
                <span class="ct-choice-eco {}">~{:.1} kg CO2 &middot; {}</span>
            </button>"#,
            i, choice.label, eco_level.1, choice.co2, eco_level.0
        ));
    }

    let html = format!(r#"
        <div class="ct-header">
            <div class="ct-stage-info">
                <span class="ct-stage-num">Stage {} of 6</span>
                <span class="ct-stage-title">{}</span>
            </div>
            <div class="ct-co2-display">{:.1} <span class="ct-co2-unit">kg CO2</span></div>
            <div class="ct-progress-bar"><div class="ct-progress-fill" style="width: {:.1}%;"></div></div>
        </div>
        <div class="ct-content">
            <div class="ct-stage-icon">{}</div>
            <p class="ct-description">{}</p>
            <div class="ct-choices">{}</div>
        </div>
    "#, state.current_stage + 1, stage.title,
        state.total_co2, bar_pct,
        stage.icon, stage.description, choices_html);
    container.set_inner_html(&html);
}

fn attach_listeners(state: Rc<RefCell<GameState>>, document: &Document, container: &Element) {
    let doc = document.clone();
    let cont = container.clone();

    // Choice buttons
    let buttons = document.query_selector_all(".ct-choice-btn").unwrap();
    for i in 0..buttons.length() {
        let node = buttons.item(i).unwrap();
        let el: HtmlElement = node.dyn_into().unwrap();
        let idx_str = el.get_attribute("data-choice").unwrap_or_default();
        let idx: usize = idx_str.parse().unwrap_or(0);
        let s = state.clone();
        let d = doc.clone();
        let c = cont.clone();
        let cb = Closure::wrap(Box::new(move || {
            let mut gs = s.borrow_mut();
            let choice = gs.stages[gs.current_stage].choices[idx].clone();
            gs.total_co2 += choice.co2;
            gs.last_choice_co2 = choice.co2;
            gs.last_choice_fact = choice.fact.to_string();
            gs.showing_result = true;
            render(&gs, &d, &c);
            drop(gs);
            attach_listeners(s.clone(), &d, &c);
        }) as Box<dyn Fn()>);
        el.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // Continue button
    if let Ok(Some(btn)) = document.query_selector("#ct-continue") {
        let el: HtmlElement = btn.dyn_into().unwrap();
        let s = state.clone();
        let d = doc.clone();
        let c = cont.clone();
        let cb = Closure::wrap(Box::new(move || {
            let mut gs = s.borrow_mut();
            gs.showing_result = false;
            gs.current_stage += 1;
            if gs.current_stage >= gs.stages.len() {
                gs.game_over = true;
            }
            render(&gs, &d, &c);
            drop(gs);
            attach_listeners(s.clone(), &d, &c);
        }) as Box<dyn Fn()>);
        el.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }

    // Play Again button
    if let Ok(Some(btn)) = document.query_selector("#ct-play-again") {
        let el: HtmlElement = btn.dyn_into().unwrap();
        let s = state.clone();
        let d = doc.clone();
        let c = cont.clone();
        let cb = Closure::wrap(Box::new(move || {
            let mut gs = s.borrow_mut();
            *gs = GameState::new();
            render(&gs, &d, &c);
            drop(gs);
            attach_listeners(s.clone(), &d, &c);
        }) as Box<dyn Fn()>);
        el.set_onclick(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let container = document.get_element_by_id("carbon-trail-game")
        .expect("Need element with id 'carbon-trail-game'");

    let state = Rc::new(RefCell::new(GameState::new()));

    render(&state.borrow(), &document, &container);
    attach_listeners(state, &document, &container);

    Ok(())
}
