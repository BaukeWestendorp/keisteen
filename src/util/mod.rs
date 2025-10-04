#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum IntProvider {
    #[serde(rename = "minecraft:constant")]
    Constant { value: i32 },
    #[serde(rename = "minecraft:uniform")]
    Uniform { min_inclusive: i32, max_inclusive: i32 },
    #[serde(rename = "minecraft:biased_to_bottom")]
    BiasedToBottom { min_inclusive: i32, max_inclusive: i32 },
    #[serde(rename = "minecraft:clamped")]
    Clamped { min_inclusive: i32, max_inclusive: i32, source: Box<IntProvider> },
    #[serde(rename = "minecraft:clamped_normal")]
    ClampedNormal { mean: f32, deviation: f32, min_inclusive: i32, max_inclusive: i32 },
    #[serde(rename = "minecraft:weighted_list")]
    WeightedList { distribution: Vec<WeightedEntry> },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeightedEntry {
    data: IntProvider,
    weight: i32,
}
