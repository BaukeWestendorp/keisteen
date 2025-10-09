pub trait Predicate<T> {
    fn test(&self, t: &T) -> bool;
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum IntProvider {
    #[serde(rename = "minecraft:uniform")]
    Uniform { min_inclusive: i32, max_inclusive: i32 },
    #[serde(rename = "minecraft:biased_to_bottom")]
    BiasedToBottom { min_inclusive: i32, max_inclusive: i32 },
    #[serde(rename = "minecraft:clamped")]
    Clamped { min_inclusive: i32, max_inclusive: i32, source: Box<IntProvider> },
    #[serde(rename = "minecraft:clamped_normal")]
    ClampedNormal { mean: f64, deviation: f64, min_inclusive: i32, max_inclusive: i32 },
    #[serde(rename = "minecraft:weighted_list")]
    WeigthedList(Vec<WeightedEntry<i32>>),
    #[serde(untagged)]
    Constant(i32),
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightedEntry<T> {
    pub data: T,
    pub weight: i32,
}
