use serde::Deserialize;
use serde::Serialize;

pub type Data = Vec<Movement>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    pub end_time: String,
    pub start_time: String,
    pub visit: Option<Visit>,
    pub activity: Option<Activity>,
    #[serde(default)]
    pub timeline_path: Vec<TimelinePath>,
    pub timeline_memory: Option<TimelineMemory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visit {
    pub hierarchy_level: String,
    pub top_candidate: TopCandidate,
    pub probability: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopCandidate {
    pub probability: String,
    pub semantic_type: String,
    #[serde(rename = "placeID")]
    pub place_id: String,
    pub place_location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub probability: Option<String>,
    pub end: String,
    pub top_candidate: TopCandidate2,
    pub distance_meters: String,
    pub start: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopCandidate2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub probability: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelinePath {
    pub point: String,
    pub duration_minutes_offset_from_start_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineMemory {
    pub distance_from_origin_kms: String,
    #[serde(default)]
    pub destinations: Vec<Destination>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    pub identifier: String,
}
