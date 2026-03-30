use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Choice {
    pub label: String,
    pub co2: f64,
    pub fact: String,
    pub eco_level: String, // "low", "med", "high"
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StageInfo {
    pub title: String,
    pub description: String,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub current_stage: usize,
    pub total_co2: f64,
    pub choices_made: Vec<usize>,
    pub co2_per_stage: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct ChoiceResult {
    pub state: GameState,
    pub co2_added: f64,
    pub fact: String,
    pub choice_label: String,
    pub game_over: bool,
    pub total_stages: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GradeInfo {
    pub grade: String,
    pub color: String,
    pub message: String,
}

fn eco_level(co2: f64) -> String {
    if co2 <= 1.5 { "low".into() }
    else if co2 <= 4.0 { "med".into() }
    else { "high".into() }
}

fn stages() -> Vec<StageInfo> {
    vec![
        StageInfo {
            title: "Raw Material".into(),
            description: "Every garment begins with a fiber. The choice of raw material sets the foundation for the entire carbon footprint.".into(),
            choices: vec![
                Choice { label: "Organic Cotton".into(), co2: 2.5, fact: "Organic cotton uses 91% less water and no synthetic pesticides. It produces about 46% less CO2 than conventional cotton farming.".into(), eco_level: eco_level(2.5) },
                Choice { label: "Conventional Cotton".into(), co2: 5.0, fact: "Conventional cotton uses heavy pesticides and fertilizers. It takes about 2,700 liters of water to make one cotton t-shirt.".into(), eco_level: eco_level(5.0) },
                Choice { label: "Polyester".into(), co2: 7.5, fact: "Polyester is made from petroleum and takes 200+ years to decompose. Production emits 3x more CO2 than cotton.".into(), eco_level: eco_level(7.5) },
            ],
        },
        StageInfo {
            title: "Manufacturing".into(),
            description: "Your fiber heads to the factory to be spun, woven, and cut into a garment. The type of factory matters enormously for emissions.".into(),
            choices: vec![
                Choice { label: "Renewable Energy Factory".into(), co2: 1.5, fact: "Factories powered by solar and wind can reduce manufacturing emissions by up to 80%. Fair wages and safe conditions are standard.".into(), eco_level: eco_level(1.5) },
                Choice { label: "Standard Factory".into(), co2: 4.0, fact: "Most garment factories run on coal or natural gas. The fashion industry accounts for 10% of global carbon emissions.".into(), eco_level: eco_level(4.0) },
                Choice { label: "Unregulated Sweatshop".into(), co2: 6.5, fact: "Unregulated factories often use the dirtiest energy sources, dump waste illegally, and exploit workers with unsafe conditions.".into(), eco_level: eco_level(6.5) },
            ],
        },
        StageInfo {
            title: "Dyeing & Treatment".into(),
            description: "Time to add color and finish. Textile dyeing is one of the most polluting steps, responsible for 20% of global water pollution.".into(),
            choices: vec![
                Choice { label: "Natural Dyes".into(), co2: 1.0, fact: "Plant-based dyes from indigo, turmeric, and madder root are biodegradable and non-toxic. They produce zero synthetic waste.".into(), eco_level: eco_level(1.0) },
                Choice { label: "Low-Impact Synthetic".into(), co2: 2.5, fact: "Low-impact dyes have a higher absorption rate (95%+), meaning less rinse water and chemical waste.".into(), eco_level: eco_level(2.5) },
                Choice { label: "Conventional Chemical Dyes".into(), co2: 5.0, fact: "Conventional dyeing uses 8,000+ synthetic chemicals. Wastewater is often dumped into rivers, poisoning ecosystems.".into(), eco_level: eco_level(5.0) },
            ],
        },
        StageInfo {
            title: "Shipping".into(),
            description: "Your garment is ready to travel to stores worldwide. How it gets there makes a huge difference in its carbon footprint.".into(),
            choices: vec![
                Choice { label: "Local / Train".into(), co2: 0.5, fact: "Locally made garments travel minimal distances. Rail freight produces 75% fewer emissions than trucks and 95% less than air.".into(), eco_level: eco_level(0.5) },
                Choice { label: "Container Ship".into(), co2: 3.0, fact: "90% of world trade travels by sea. A single large container ship can emit as much pollution as 50 million cars in a year.".into(), eco_level: eco_level(3.0) },
                Choice { label: "Air Freight".into(), co2: 8.0, fact: "Air freight produces 50x more CO2 per km than shipping by sea. Fast fashion brands increasingly use air freight for trends.".into(), eco_level: eco_level(8.0) },
            ],
        },
        StageInfo {
            title: "Consumer Use".into(),
            description: "The garment is yours! How you care for it has a surprising impact. About 25% of a garment's carbon footprint comes from washing and drying.".into(),
            choices: vec![
                Choice { label: "Cold Wash / Line Dry".into(), co2: 1.0, fact: "Washing at 30C instead of 60C uses 57% less energy. Line drying saves about 2.4 kg of CO2 per load vs machine drying.".into(), eco_level: eco_level(1.0) },
                Choice { label: "Warm Wash / Tumble Dry".into(), co2: 3.5, fact: "A tumble dryer is one of the most energy-hungry home appliances. The average household does 400 loads per year.".into(), eco_level: eco_level(3.5) },
                Choice { label: "Hot Wash / Dry Clean".into(), co2: 6.0, fact: "Hot washing degrades fabrics faster, shortening garment life. Dry cleaning uses perchloroethylene, a toxic chemical.".into(), eco_level: eco_level(6.0) },
            ],
        },
        StageInfo {
            title: "End of Life".into(),
            description: "Every garment eventually reaches the end of its wearable life. What happens next determines whether it becomes waste or gets a second chance.".into(),
            choices: vec![
                Choice { label: "Repair & Rewear".into(), co2: 0.5, fact: "Extending a garment's life by just 9 months reduces its carbon, water, and waste footprint by 20-30%.".into(), eco_level: eco_level(0.5) },
                Choice { label: "Donate / Thrift".into(), co2: 1.5, fact: "Donating keeps clothes in circulation. However, only 10-20% of donated clothes are resold locally.".into(), eco_level: eco_level(1.5) },
                Choice { label: "Recycle".into(), co2: 3.0, fact: "Less than 1% of clothing is recycled into new garments. Most textile recycling downcycles into insulation or rags.".into(), eco_level: eco_level(3.0) },
                Choice { label: "Landfill".into(), co2: 6.0, fact: "92 million tons of textiles end up in landfills yearly. Synthetic fabrics take 200+ years to decompose.".into(), eco_level: eco_level(6.0) },
            ],
        },
    ]
}

#[wasm_bindgen]
pub fn new_game() -> String {
    let state = GameState {
        current_stage: 0,
        total_co2: 0.0,
        choices_made: vec![],
        co2_per_stage: vec![],
    };
    serde_json::to_string(&state).unwrap()
}

#[wasm_bindgen]
pub fn get_stage_info(stage_index: usize) -> String {
    let all = stages();
    if stage_index < all.len() {
        serde_json::to_string(&all[stage_index]).unwrap()
    } else {
        "null".into()
    }
}

#[wasm_bindgen]
pub fn get_total_stages() -> usize {
    stages().len()
}

#[wasm_bindgen]
pub fn make_choice(state_json: &str, stage_index: usize, choice_index: usize) -> String {
    let mut state: GameState = serde_json::from_str(state_json).unwrap();
    let all = stages();
    let stage = &all[stage_index];
    let choice = &stage.choices[choice_index];

    state.total_co2 += choice.co2;
    state.choices_made.push(choice_index);
    state.co2_per_stage.push(choice.co2);
    state.current_stage = stage_index + 1;

    let game_over = state.current_stage >= all.len();

    let result = ChoiceResult {
        state,
        co2_added: choice.co2,
        fact: choice.fact.clone(),
        choice_label: choice.label.clone(),
        game_over,
        total_stages: all.len(),
    };
    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_grade(total_co2: f64) -> String {
    let info = if total_co2 <= 10.0 {
        GradeInfo { grade: "A".into(), color: "#22C55E".into(), message: "Exceptional! Your garment has a minimal carbon footprint.".into() }
    } else if total_co2 <= 16.0 {
        GradeInfo { grade: "B".into(), color: "#84CC16".into(), message: "Great job! Your choices significantly reduced emissions.".into() }
    } else if total_co2 <= 22.0 {
        GradeInfo { grade: "C".into(), color: "#EAB308".into(), message: "Average impact. There's room for improvement.".into() }
    } else if total_co2 <= 30.0 {
        GradeInfo { grade: "D".into(), color: "#F97316".into(), message: "High impact. Consider more sustainable alternatives.".into() }
    } else {
        GradeInfo { grade: "F".into(), color: "#EF4444".into(), message: "Very high impact. This garment leaves a heavy carbon trail.".into() }
    };
    serde_json::to_string(&info).unwrap()
}

#[wasm_bindgen]
pub fn get_industry_avg() -> f64 {
    33.4
}

#[wasm_bindgen]
pub fn get_all_choice_labels() -> String {
    let all = stages();
    let labels: Vec<Vec<String>> = all.iter().map(|s| s.choices.iter().map(|c| c.label.clone()).collect()).collect();
    serde_json::to_string(&labels).unwrap()
}
