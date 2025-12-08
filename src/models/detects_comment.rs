

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetectsComment {
    #[serde(rename = "falcon_user_id")]
    pub falcon_user_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl DetectsComment {
    pub fn new(
        falcon_user_id: String,
        timestamp: String,
        value: String
    ) -> DetectsComment {
        DetectsComment {
            falcon_user_id,
            timestamp,
            value
        }
    }
}